extern crate discord;

use self::discord::Discord;

pub const BOTSYMBOL: & str = "~";

pub fn get_name_discrim( message: & discord::model::Message ) -> String
{
	format!( "{}#{}", message.author.name, message.author.discriminator )
}

pub fn send_disc_message( discord: & Discord,
		      message: & discord::model::Message,
		      msg: & str )
{
	let _ = discord.send_message( message.channel_id, msg, "", false );
}

pub fn get_first_whitespace( text: & String ) -> i64
{
	for ( i, c ) in text.chars().enumerate() {
		if c == ' ' {
			return i as i64
		}
	}

	return -1
}
