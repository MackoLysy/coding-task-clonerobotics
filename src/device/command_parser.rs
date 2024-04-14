#[derive(Debug)]
pub enum CommandType {
    None,
    Start,
    Stop,
    Config,
}
use log::*;

pub fn parse_command(data: String) -> CommandType {
    if data.starts_with("$0") && data.len() < 3 {
        return CommandType::Start;
    }
    if data.starts_with("$1") && data.len() < 3 {
        return CommandType::Stop;
    }
    if data.starts_with("$2") {
        return CommandType::Config;
    }
    CommandType::None
}

pub fn parse_config(data: String) -> Option<(u8, bool)> {
    let binding = data.replace('\n', "");
    trace!("Parsing config msg: {}", data);
    let datas: Vec<&str> = binding.split(',').collect();
    if datas.len() > 2 {
        let freq = datas[1].parse::<u8>();
        let mut led_val = false;
        match datas[2] {
            "1" => {
                led_val = true;
            }
            "0" => {
                led_val = false;
            }
            _ => {
                return None;
            }
        };

        if freq.is_ok() {
            trace!("freq:{:#?}, led:{}", freq, led_val);
            return Some((freq.unwrap(), led_val));
        } else {
            error!("failed to parse config msg, {}", data);
        }
    }
    None
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_invalid_number() {
        let result = parse_command("$6\n".to_string());
        assert_eq!(matches!(result, CommandType::None), true);
    }

    #[test]
    fn parse_invalid_string_cmd() {
        let result = parse_command("$invalid\n".to_string());
        assert_eq!(matches!(result, CommandType::None), true);
    }

    #[test]
    fn parse_start_valid_cmd() {
        let result = parse_command("$0".to_string());
        assert_eq!(matches!(result, CommandType::Start), true);
    }
    #[test]
    fn parse_stop_cmd() {
        let result = parse_command("$1".to_string());
        assert_eq!(matches!(result, CommandType::Stop), true);
    }
    #[test]
    fn parse_config_cmd() {
        let result = parse_command("$2".to_string());
        assert_eq!(matches!(result, CommandType::Config), true);
    }
    #[test]
    fn parse_config_with_proper_data() {
        let result = parse_config("$2,12,0\n".to_string());
        assert_eq!(result.is_some(), true);
    }
    #[test]
    fn parse_config_with_invalid_led_data() {
        let result = parse_config("$2,12,12\n".to_string());
        assert_eq!(result.is_some(), false);
    }
    #[test]
    fn parse_config_with_invalid_freq_data() {
        let result = parse_config("$2,asdas,12\n".to_string());
        assert_eq!(result.is_some(), false);
    }
}
