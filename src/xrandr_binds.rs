use std::process::Command;


pub fn is_xorg_session () -> bool {
    match std::env::var("XDG_SESSION_TYPE") {
        Ok(session_type) => {
            if session_type.contains("x11") {return true;} else {return false;}
        },
        Err(_e) => {return false;},
    }
}

// xrandr --listactivemonitors | grep ": +" | grep -o '[^ ]\+$'
 
pub fn list_enable_monitors () -> Vec<String> {
    let mut output: Vec<String> = vec![];

    let command = 
        Command::new("xrandr")
            .arg("--listactivemonitors")
            .output()
            .expect("failed to execute process");

    let s = match str::from_utf8(&command.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let svec: Vec<&str> = s.split('\n').collect();

    for m in svec {
        if m.contains("+") {
            if let Some(pos) = m.rfind(' ') {
                let slice = m[pos..].trim();
                output.push(slice.to_owned());
            }

        }
    }
    output
}

// xrandr --output $item --brightness $result_brightness
pub fn set_brightness (monitor: &str, value: &str) {
    let _command = 
        Command::new("xrandr")
            .arg("--output")
            .arg(monitor)
            .arg("--brightness")
            .arg(value)
            .output()
            .expect("failed to execute process");
}

// xrandr --output HDMI-0 --brightness 0.8 --gamma 1.0:0.7:0.5
pub fn set_gamma (monitor: &str, brightness: &str, r: &str,  g: &str, b: &str) {
    let _command = 
        Command::new("xrandr")
            .arg("--output")
            .arg(monitor)
            .arg("--brightness")
            .arg(brightness)
            .arg("--gamma")
            .arg(format!("{}:{}:{}", r, g, b)) 
            .output()
            .expect("failed to execute process");

}

// xrandr --verbose > brightness

pub fn get_brightness (monitor: &str) -> f64 {
    let _command = Command::new("xrandr")
        .arg("--verbose")
        .output()
        .expect("failed to execute process");

    let monitor = monitor;

    // dbg!(&_command);
    let s = match str::from_utf8(&_command.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    fn line_of_monitor(xrandverbose: &str, monitor: &str) -> usize {
        let mut i = 0;
        for line in xrandverbose.lines() {
            if line.contains(monitor) {
                return i;
            }
            i += 1;
        }
        i
    }

    let target_line = s.lines().nth(line_of_monitor (s, monitor)+5).unwrap().split(":").nth(1).unwrap().trim();
    let output: f64 = target_line.parse().unwrap();
    output
    
}

// xrandr --verbose > gamma

pub fn get_gamma (monitor: &str) -> (f64, f64, f64){
    let _command = Command::new("xrandr")
    .arg("--verbose")
    .output()
    .expect("failed to execute process");

    let monitor = monitor;

    let s = match str::from_utf8(&_command.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    fn line_of_monitor(xrandverbose: &str, monitor: &str) -> usize {
        let mut i = 0;
        for line in xrandverbose.lines() {
            if line.contains(monitor) {
                return i;
            }
            i += 1;
        }
        i
    }

    let mut target_line = s.lines().nth(line_of_monitor (s, monitor)+4).unwrap().split(":      ").nth(1).unwrap().split(":");

    // This is better than using "nth"
    let red = target_line.next().unwrap().to_owned();
    let green = target_line.next().unwrap().to_owned();
    let blue = target_line.next().unwrap().to_owned();

    let red: f64 = red.parse().unwrap() ;
    let green: f64 = green.parse().unwrap();
    let blue: f64 = blue.parse().unwrap();

    (red, green, blue)
}
