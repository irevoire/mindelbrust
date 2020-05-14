use futures::prelude::*;
use minecrust::game::player::Player;
use minecrust::game::world::World;
use minecrust::packets::ServerDescription;
use piper::Arc;
use smol::{Async, Task};
use std::net::TcpListener;
use std::time::Duration;

mod chunk;
mod mandel;

fn main() -> ! {
    let generator = chunk::MandelbrotGenerator();
    let world = smol::block_on(World::new(generator));
    eprintln!("World map generated.");

    let world: &'static World = Box::leak(Box::new(world));

    let mut server_description = ServerDescription::default();
    server_description.players = (1, 0);
    server_description.description = "Rusty Minecraft Server".to_string();
    server_description.icon = std::fs::read("./icon.png").ok();

    let listener = Async::<TcpListener>::bind("0.0.0.0:8080").unwrap();
    let mut incoming = listener.incoming();
    smol::run(async move {
        Task::spawn(world.run(Duration::from_secs(1))).detach();

        while let Some(stream) = incoming.next().await {
            println!("Go a new connection");
            let mut stream = Arc::new(stream.unwrap());
            let reader = futures::io::BufReader::new(stream.clone());
            let writer = futures::io::BufWriter::new(stream.clone());
            let player = Player::new(reader, writer, server_description.clone(), world).await;
            let player = match player {
                Err(_) => {
                    println!("It was not a minecraft client");
                    let _ = stream
                        .write(
                            br#"HTTP/1.1 200 OK
Server: nginx/1.16.1

You should connect with a minecraft client
"#,
                        )
                        .await;
                    continue;
                }
                Ok(None) => continue, // ping
                Ok(Some(player)) => player,
            };
            println!("A new player joined the server");

            Task::spawn(async move {
                world.add_player(player).await;
            })
            .detach();
        }
    });
    panic!("This should never happens");
}
