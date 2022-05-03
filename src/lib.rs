extern crate ts3plugin;

use ts3plugin::*;
mod format;
mod constants;
use constants::*;

struct Ts3JoinInfo;

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
                // Get all mention formatted version of the channels
                let connection_mention = format::format_connection(server, connection_id, api);
                let old_channel_mention = format::format_channel(server, old_channel_id, api);
                let new_channel_mention = format::format_channel(server, new_channel_id, api);

                let mut action: String = String::from("switched");
                // Format the message to be send
                match invoker {
                    Some(invoker) => {
                        let invoker_mention = format::format_mention(invoker.get_name());
                        action = format!("was moved by {}", invoker_mention);
                    }
                    _ => (),
                }
                let mut formatted_message = format!(
                    "{} {} from channel {} to {}",
                    connection_mention, action, old_channel_mention, new_channel_mention
                );
                // Wrap message in the correct styling
                formatted_message = format::format_info_message(&formatted_message);
                // Send the message to the current tab's channel
                server.print_message(formatted_message, MessageTarget::Channel);
            }
            // If the move message does not match any specified filters, do nothing
            _ => (),
        }
    }
}

fn client_connect_messenger(
    api: &mut TsApi,
    server_id: ServerId,
    connection_id: ConnectionId,
    connected: bool,
    message: String,
) {
    if let Some(server) = api.get_server(server_id) {
        // Get some english language symantics out of the way
        let mut client_action = String::from("disconnected");
        let mut channel_action = String::from("from");
        match connected {
            true => {
                client_action = String::from("connected");
                channel_action = String::from("to");
            }
            _ => (),
        }

        // Get all mention formatted version of the channels
        let connection_mention = format::format_connection(server, connection_id, api);
        match server
            .get_connection(connection_id)
            .and_then(|c| c.get_channel_id().ok())
        {
            Some(channel_id) => {
                // let channel_mention = format::format_channel(server, channel_id);
                let channel_mention = format::format_channel(server, channel_id, api);
                client_action =
                    format!("{} {} {}", client_action, channel_action, channel_mention);
            }
            _ => (),
        }

        let mut formatted_message =
            format!("{} {}", connection_mention, client_action);

        match message.is_empty() {
            false => {
                formatted_message = format!("{} ({})", formatted_message, message);
            }
            _ => (),
        }

        // Wrap message in the correct styling
        formatted_message = format::format_info_message(&formatted_message);

        server.print_message(formatted_message, MessageTarget::Channel);
    }
}

impl Plugin for Ts3JoinInfo {
    // Configure plugin info
    fn name() -> String {
        String::from(PLUGIN_NAME)
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
        api.log_or_print("Inited", PLUGIN_NAME, LogLevel::Info);
        Ok(Box::new(Ts3JoinInfo))
    }

    // Implement callbacks here

    fn shutdown(&mut self, api: &mut TsApi) {
        api.log_or_print("Shutdown", PLUGIN_NAME, LogLevel::Info);
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

    // fn connection_timeout(
    //     &mut self,
    //     api: &mut TsApi,
    //     server_id: ServerId,
    //     connection_id: ConnectionId,
    // ) {
    // }

    fn connection_changed(
        &mut self,
        api: &mut TsApi,
        server_id: ServerId,
        connection_id: ConnectionId,
        connected: bool,
        message: String,
    ) {
        client_connect_messenger(api, server_id, connection_id, connected, message);
    }
}

create_plugin!(Ts3JoinInfo);
