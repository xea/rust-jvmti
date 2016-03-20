extern crate jvmti;

#[cfg(test)]
mod tests {

    use jvmti::options::Options;

    #[test]
    fn options_return_an_options_instance_with_no_uninitialised_values() {
        let opts = Options::default();

        assert!(opts.agent_id.len() > 0);
    }

    #[test]
    fn options_can_take_an_agenetid_argument() {
        let opts = Options::parse("agentid=testid".to_string());

        assert_eq!("testid", opts.agent_id);
    }

    #[test]
    fn can_parse_several_comma_separated_arguments() {
        let opts = Options::parse("setting1,setting2,agentid=testid,setting3".to_string());

        assert_eq!("testid", opts.agent_id);
    }

    #[test]
    fn unknown_arguments_go_into_custom_args() {
        let opts = Options::parse("setting1,setting2=true,agentid=testid,setting3".to_string());

        assert_eq!(true, opts.custom_args.contains_key("setting1"));
        assert_eq!(true, opts.custom_args.contains_key("setting2"));
        assert_eq!(true, opts.custom_args.contains_key("setting3"));
    }
}
