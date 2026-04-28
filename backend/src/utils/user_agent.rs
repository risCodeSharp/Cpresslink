use once_cell::{ sync::Lazy};
use uaparser::{Parser, UserAgentParser};



#[derive(Debug)]
pub struct UserAgentInfo {
    pub device: Option<String>,
    pub os: Option<String>,
    pub browser: Option<String>,
    pub is_bot: bool,
    pub bot_name: Option<String>
}

#[derive(Debug)]
struct BotInfo {
    pub is_bot: bool,
    pub bot_name: Option<String>
}

fn detect_bot(ua: &str) -> BotInfo {
    let ua_lower = ua.to_ascii_lowercase();

    for bot in BOT_SIGNATURES {
        if ua_lower.contains(bot) {
            return BotInfo { is_bot: true,
                bot_name: Some(bot.to_string()) };
        }
    }

    BotInfo { is_bot: false, bot_name: None }
}



pub fn parse_user_agent(ua_str: &str) -> UserAgentInfo {
    
    let client: uaparser::Client = UA_PARSER.parse(&ua_str);
    
    
    let device = client.device.family;
    let os = client.os.family;
    let browser = client.user_agent.family;

    // bot detection
    let ua_lower = ua_str.to_lowercase();


    
    let BotInfo {is_bot, bot_name} = detect_bot(&ua_lower);
    UserAgentInfo { 
        device: if device == "Other" {None} else {Some(device.to_string())}, 
        os: if os == "Other" {None} else {Some(os.to_string())}, 
        browser: if browser == "Other" {None} else {Some(browser.to_string())},  
        is_bot, 
        bot_name }
}


static BOT_SIGNATURES: &[&str] = &[
    "googlebot",
    "bingbot",
    "slackbot",
    "facebookexternalhit",
    "twitterbot",
    "ahrefsbot",
    "semrushbot",
    "duckduckbot",
    "baiduspider",
    "yandexbot",
    "crawler",
    "spider",
    "bot",
];



// Load parser at the start here
static UA_PARSER: Lazy<UserAgentParser> = Lazy::new(|| {
    UserAgentParser::from_yaml("data/regexes.yaml") .expect("`data/regexes.yaml` file missing!")
});
