use std::process::Command;

fn wpctl_status() -> String {
    match Command::new("wpctl").arg("status").output() {
        Ok(output) => String::from_utf8(output.stdout).unwrap(),
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
            "hdmi1" => "DisplayPort 1",
            "hdmi2" => "DisplayPort 2",
            "hdmi3" => "DisplayPort 3",
            "headset" => "Jabra Link",
            "nb" => "HD Audio Controller",
            "flip" => "jblflip3",
            "dock" => "Dock Audio",
            _ => {
                eprintln!("unknown arg");
                std::process::exit(1);
            }
        }
    } else {
        "DisplayPort 3"
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
