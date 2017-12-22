extern crate discord;

use self::discord::Discord;

use super::common;
use super::calculator;

pub fn handle_command( discord: & Discord, message: & discord::model::Message ) -> i32
{
	// do nothing if the author of message is a bot.
	if message.author.bot { return 0 }

	// check for the special symbol marking the message to bot.
	if & message.content[ 0 .. common::BOTSYMBOL.len() ] == common::BOTSYMBOL {

		let ws = common::get_first_whitespace( & message.content );

		if ws <= 0 {

			if message.content == common::BOTSYMBOL.to_owned() + "quit" {

				if & common::get_name_discrim( message ) == "Electrux#5719" {
					common::send_disc_message(
						discord, message, "Quitting!" );

					return -1
				}
				else {
					common::send_disc_message(
						discord, message, "Unauthorized to quit!" );
				}
			}

			return 0
		}

		let ( msg_part1, msg_part2 ) = message.content.split_at( ws as usize );

		if msg_part1 == common::BOTSYMBOL.to_owned() + "calc" {

			if ws < 0 {
				common::send_disc_message(
					discord, message, "No argument specified!" );
				return 0
			}

			if msg_part2.is_empty() {
				common::send_disc_message(
					discord, message, "No argument specified!" );
				return 0
			}

			common::send_disc_message(
				discord, message, & format!( "calculating {}", msg_part2 ) );

			let res = calculator::eval_expr( msg_part2 );

			match res {
				Ok( val ) => {
					common::send_disc_message(
						discord, message,
						& format!( "Result: {}", val ) );
				}
				Err( err ) => {
					common::send_disc_message(
						discord, message,
						& format!( "Error: {}", err ) );
				}
			}
		}
	}
	return 0
}
