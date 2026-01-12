mod http;

pub fn parse_args(args: Vec<String>) -> Result<HTTPConnectionInfo, String> {
    if args.len() < 1 {
        return Err("Not enough arguments");
    }
}
