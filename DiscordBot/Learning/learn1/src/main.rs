extern crate discord;

use discord::Discord;
use discord::model::Event;

fn get_name_discrim( message: & discord::model::Message ) -> String
{
	format!( "{}#{}", message.author.name, message.author.discriminator )
}

fn send_disc_message( discord: & Discord, message: & discord::model::Message, msg: & str )
{
	let _ = discord.send_message( message.channel_id, msg, "", false );
}

fn handle_command( discord: & Discord, message: & discord::model::Message ) -> i32
{
	if message.author.bot { return 0 }

	if & message.content[ 0 .. 2 ] == "#!" {

		if message.content == "#!test" {
			send_disc_message( discord, message, "This is a response to test" );
		}
		else if message.content == "#!quit" {

			if & get_name_discrim( message ) == "Electrux#5719" {
				send_disc_message( discord, message, "Quitting!" );

				return -1
			}
			else {
				send_disc_message( discord, message, "Unauthorized to quit!" );
			}
		}
	}
	else {
		println!( "{}#{} says: {}" ,
				 message.author.name,
				 message.author.discriminator,
				 message.content );
	}

	return 0
}

fn match_connection_recv_event( discord: & Discord, connection: & mut discord::Connection ) -> i32
{
	match connection.recv_event() {
		Ok( Event::MessageCreate( message ) ) => {

			if handle_command( discord, & message ) < 0 {
				return -1
			}
		}

		Ok( _ ) => {}

		Err( discord::Error::Closed( code, body ) ) => {
			println!( "Gateway closed with code: {:?} and body: {}", code, body );

			return -1
		}

		Err( err ) => {
			println!( "Received error: {}", err );
		}
	}

	return 0
}

fn main()
{
	let discord = Discord::from_bot_token(
		& std::env::var( "DISCORD_TOKEN" ).expect( "No env var for DISCORD_TOKEN" )
	).expect( "Unable to login bot" );

	let ( mut connection, _ ) = discord.connect().expect( "Unable to connect" );

	println!( "Bot is connected!" );

	loop {
		if match_connection_recv_event( & discord, & mut connection ) < 0 {
			break
		}
	}
}
