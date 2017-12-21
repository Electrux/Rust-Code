extern crate discord;
extern crate calculator;

use discord::Discord;
use discord::model::Event;

const BOTSYMBOL: & str = "##";

fn get_name_discrim( message: & discord::model::Message ) -> String
{
	format!( "{}#{}", message.author.name, message.author.discriminator )
}

fn send_disc_message( discord: & Discord, message: & discord::model::Message, msg: & str )
{
	let _ = discord.send_message( message.channel_id, msg, "", false );
}

fn get_first_whitespace( text: & String ) -> i64
{
	for ( i, c ) in text.chars().enumerate() {
		if c == ' ' {
			return i as i64
		}
	}

	return -1
}

fn handle_command( discord: & Discord, message: & discord::model::Message ) -> i32
{
	// do nothing if the author of message is a bot.
	if message.author.bot { return 0 }

	// check for the special symbol marking the message to bot.
	if & message.content[ 0 .. BOTSYMBOL.len() ] == BOTSYMBOL {

		let ws = get_first_whitespace( & message.content );

		if ws <= 0 {

			if message.content == BOTSYMBOL.to_owned() + "quit" {

				if & get_name_discrim( message ) == "Electrux#5719" {
					send_disc_message( discord, message, "Quitting!" );

					return -1
				}
				else {
					send_disc_message(
						discord, message, "Unauthorized to quit!" );
				}
			}

			return 0
		}

		let ( msg_part1, msg_part2 ) = message.content.split_at( ws as usize );

		if msg_part1 == BOTSYMBOL.to_owned() + "calc" {

			if ws < 0 {
				send_disc_message( discord, message, "No argument specified!" );
				return 0
			}

			if msg_part2.is_empty() {
				send_disc_message( discord, message, "No argument specified!" );
				return 0
			}

			send_disc_message(
				discord, message, & format!( "calculating {}", msg_part2 ) );

			let res = calculator::eval_expr( msg_part2 );

			match res {
				Ok( val ) => {
					send_disc_message(
						discord, message,
						& format!( "Result: {}", val ) );
				}
				Err( err ) => {
					send_disc_message(
						discord, message,
						& format!( "Error: {}", err ) );
				}
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

fn match_connection_recv_event( discord: & Discord,
				connection: & mut discord::Connection ) -> i32
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
