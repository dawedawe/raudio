use std::process::Command;

fn wpctl_status() -> String {
    match Command::new("wpctl").arg("status").output() {
        Ok(output) => {
            let out = String::from_utf8(output.stdout).unwrap();
            out
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn find_sink_id(status: &str, sink: &str) -> Option<String> {
    status
        .lines()
        .skip_while(|line| !line.contains("Sinks:"))
        .find(|line| line.contains(sink))
        .map(|line| {
            let re = regex::Regex::new(r"\d+").unwrap();
            re.find(line).unwrap().as_str().to_string()
        })
}

fn wpctl_set_default(id: &str) {
    match Command::new("wpctl").arg("set-default").arg(id).status() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn main() {
    let sink = if let Some(arg1) = std::env::args().nth(1) {
        match arg1.as_str() {
            "hdmi" => "DisplayPort 1",
            "headset" => "Jabra Link",
            "nb" => "HD Audio Controller",
            _ => {
                eprintln!("unknown arg");
                std::process::exit(1);
            }
        }
    } else {
        "DisplayPort 1"
    };

    let status = wpctl_status();
    match find_sink_id(&status, sink) {
        Some(id) => {
            println!("{} found at id: {}", sink, id);
            wpctl_set_default(&id);
        }
        None => {
            println!("{} not found", sink);
        }
    }
}
