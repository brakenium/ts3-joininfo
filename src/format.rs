use chrono::prelude::Local;
use ts3plugin::*;

pub fn format_mention(text: &str) -> String {
    // Use BBCode formatting to get the darkblue and bold styling of the default join messages
    return format!("[color=#002f5d][b]\"{}\"[/b][/color]", text);
}

pub fn mention_connection(conn: &Connection) -> String {
    if let Ok(name) = conn.get_name() {
        if let Ok(uid) = conn.get_uid() {
            return format!("[URL=client://{}~{}]{}[/URL]", uid, name, name);
        }
        else {
            return format_mention(name);
        }
    }
    String::from("Unkown client")
}

pub fn mention_channel(conn: &Channel) -> String {
    if let Ok(name) = conn.get_name() {
        let channel_id = conn.get_id();
        return format!("[URL=channel://{:?}~{}]{}[/URL]", channel_id, name, name);
    }
    String::from("Unkown client")
}

pub fn format_connection(server: &Server, connection_id: ConnectionId) -> String {
    match server.get_connection(connection_id) {
        Some(conn) => {
            let connection_mention = mention_connection(conn);
            return format_mention(connection_mention.as_str());
        },
        _ => (),
    }
    String::from("Unkown client")
}

pub fn format_channel(server: &Server, channel_id: ChannelId) -> String {
    match server.get_channel(channel_id) {
        Some(channel) => {
            let connection_mention = mention_channel(channel);
            return format_mention(connection_mention.as_str());
        },
        _ => (),
    }
    String::from("Unkown client")
}

pub fn format_info_message(text: &str) -> String {
    // Format time in the user's locale, either 12h or 24h clock
    let formatted_time = Local::now().format("%X");
    // wrap the message in the correct colour and add the time like <13:20:00>
    return format!("[color=#33597d]<{}> {}[/color]", formatted_time, text);
}
