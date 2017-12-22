extern crate discord;
extern crate electrux_bot;

use discord::Discord;
use discord::model::Event;

use electrux_bot::command_mgr;

fn main()
{
	let discord = Discord::from_bot_token(
		& std::env::var( "DISCORD_TOKEN" ).expect( "No env var for DISCORD_TOKEN" )
	).expect( "Unable to login bot" );

	let ( mut connection, _ ) = discord.connect().expect( "Unable to connect" );

	println!( "Bot is connected!" );

	loop {
		match connection.recv_event() {

			Ok( Event::MessageCreate( message ) ) => {

				if command_mgr::handle_command( & discord, & message ) < 0 {
					break
				}
			}

			Ok( _ ) => {}

			Err( discord::Error::Closed( code, body ) ) => {
				println!( "Gateway closed with code: {:?} and body: {}",
					   code, body );

				break
			}

			Err( err ) => {
				println!( "Received error: {}", err );
			}
		}
	}
}
