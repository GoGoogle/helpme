pub fn apply_cli_overrides(cfg: &mut toml::Value) {
    let args: Vec<String> = std::env::args().collect();
    let mut i = 0;
    while i + 1 < args.len() {
        match args[i].as_str() {
            "-port" => {
                if let Ok(p) = args[i+1].parse::<u16>() {
                    cfg["direct-access-port"] = toml::Value::from(p);
                }
                i += 2;
            }
            "-pw" => {
                cfg["default-connect-password"] = toml::Value::from(args[i+1].clone());
                i += 2;
            }
            _ => i += 1,
        }
    }
}
