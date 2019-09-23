use toml::Value;

pub fn get_path(url: &str, config: toml::Value) -> String {
    let mut path: String = String::new();
    let mut url_split: Vec<&str> = url.split("/").collect();
    let path = expand_path(url_split, config);

    path
}

fn expand_path(token: Vec<&str>, config: toml::Value) -> String {
    let mut current = config;
    let mut path: String = String::new();

    for key in token {
        current = match current.get(key) {
            None => break,
            Some(x) => {
                if let Some(value) = x.get("expand") {
                    path.push_str(value.as_str().unwrap());
                    path.push_str("/");
                };
                x.clone()
            },
        };
    }

    path
}

