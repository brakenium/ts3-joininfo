use chrono::prelude::{Local};

pub fn format_mention(text: &str) -> String {
    // Use BBCode formatting to get the darkblue and bold styling of the default join messages
    return format!("[color=#002f5d][b]\"{}\"[/b][/color]", text);
}

pub fn format_info_message(text: &str) -> String {
    // Format time in the user's locale, either 12h or 24h clock
    let formatted_time = Local::now().format("%X");
    // wrap the message in the correct colour and add the time like <13:20:00>
    return format!("[color=#33597d]<{}> {}[/color]", formatted_time, text)
}