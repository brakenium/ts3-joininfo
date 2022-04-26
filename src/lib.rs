extern crate ts3plugin;

use ts3plugin::*;
mod util;

struct Ts3JoinInfo;

// Internal function for getting the channel name
fn intern_get_connection_name(connection: &Connection) -> &str {
    let result = connection
        .get_phonetic_name()
        .ok()
        .and_then(|s| if s.is_empty() { None } else { Some(s.as_str()) })
        .or_else(|| connection.get_name().ok().map(|s| s.as_str()))
        .unwrap_or("unknown");
    // Take name until the first space
    result.find(' ').map_or(result, |i| result.split_at(i).0)
}

/// Get the name or phonetic name of a connection.
fn get_connection_name<'a>(server: &'a Server, connection_id: ConnectionId) -> &'a str {
    server
        .get_connection(connection_id)
        .map_or("unknown", |connection| {
            intern_get_connection_name(connection)
        })
}

fn client_moved_messenger(
    api: &mut TsApi,
    server_id: ServerId,
    connection_id: ConnectionId,
    old_channel_id: ChannelId,
    new_channel_id: ChannelId,
    invoker: Option<Invoker>,
) {
    if let Some(server) = api.get_server(server_id) {
        let own_connection_id = server.get_own_connection_id();
        // Make sure it is not our own client
        // Get our own connection id
        let own_connection = own_connection_id
            .ok()
            .and_then(|c| server.get_connection(c));
        let own_channel_id = own_connection.and_then(|c| c.get_channel_id().ok());

        // Handle other clients joining the client's channel
        match own_channel_id {
            // Format a message that looks like a server wide move message
            // Only if either old or new channel is the client's current channel
            Some(channel_id) if [old_channel_id, new_channel_id].contains(&channel_id) => {
                // Get names of all channels that need to be mentioned
                let connection_name = get_connection_name(server, connection_id);
                let new_channel_name = get_channel_name(server, new_channel_id);
                let old_channel_name = get_channel_name(server, old_channel_id);

                // Get all mention formatted version of the channels
                let connection_mention = util::format_mention(connection_name);
                let old_channel_mention = util::format_mention(old_channel_name);
                let new_channel_mention = util::format_mention(new_channel_name);

                let mut action: String = String::from("switched");
                // Format the message to be send
                match invoker {
                    Some(invoker) => {
                        let invoker_mention = util::format_mention(invoker.get_name());
                        action = format!("was moved by {}", invoker_mention);
                    }
                    _ => (),
                }
                let mut formatted_message = format!(
                    "{} {} from channel {} to {}",
                    connection_mention, action, old_channel_mention, new_channel_mention
                );
                // Wrap message in the correct styling
                formatted_message = util::format_info_message(&formatted_message);
                // Send the message to the current tab's channel
                server.print_message(formatted_message, MessageTarget::Channel);
            }
            // If the move message does not match any specified filters, do nothing
            _ => (),
        }
    }
}

/// Get the name or phonetic name of a channel.
fn get_channel_name<'a>(server: &'a Server, channel_id: ChannelId) -> &'a str {
    server
        .get_channel(channel_id)
        .and_then(|c| {
            c.get_phonetic_name()
                .ok()
                .and_then(|s| if s.is_empty() { None } else { Some(s) })
                .or_else(|| c.get_name().ok())
        })
        .map(|s| s.as_str())
        .unwrap_or("unknown channel")
}

impl Plugin for Ts3JoinInfo {
    // Configure plugin info
    fn name() -> String {
        String::from("Channel Join Info")
    }
    fn version() -> String {
        String::from("0.1.0")
    }
    fn author() -> String {
        String::from("Brakenium")
    }
    fn description() -> String {
        String::from("A plugin that will display user join messages in the channel chat.")
    }
    fn new(api: &mut TsApi) -> Result<Box<Ts3JoinInfo>, InitError> {
        api.log_or_print("Inited", "ts3-joininfo", LogLevel::Info);
        Ok(Box::new(Ts3JoinInfo))
    }

    // Implement callbacks here

    fn shutdown(&mut self, api: &mut TsApi) {
        api.log_or_print("Shutdown", "ts3-joininfo", LogLevel::Info);
    }

    // TODO: Add connection_moved callback

    fn connection_move(
        &mut self,
        api: &mut TsApi,
        server_id: ServerId,
        connection_id: ConnectionId,
        old_channel_id: ChannelId,
        new_channel_id: ChannelId,
        _visibility: Visibility,
    ) {
        client_moved_messenger(
            api,
            server_id,
            connection_id,
            old_channel_id,
            new_channel_id,
            None,
        );
    }

    fn connection_moved(
        &mut self,
        api: &mut TsApi,
        server_id: ServerId,
        connection_id: ConnectionId,
        old_channel_id: ChannelId,
        new_channel_id: ChannelId,
        _visibility: Visibility,
        invoker: Invoker,
    ) {
        client_moved_messenger(
            api,
            server_id,
            connection_id,
            old_channel_id,
            new_channel_id,
            Some(invoker),
        );
    }
}

create_plugin!(Ts3JoinInfo);
