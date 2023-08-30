use std::process::Command;

#[derive(Debug)]
struct Service {}

pub enum Services {
    // OpenRc,
    // SoystemD
}

impl Services {
    pub fn guess() -> impl ServiceBackend {
        OpenRcServicer
    }
}

pub trait ServiceBackend {
    fn enable(&mut self, name: &[&str]);

    fn disable(&mut self, name: String);

    fn list_all_enabled(&mut self) -> Vec<String>;

    fn list_leaves_enabled(&mut self) -> Vec<String>;
}

struct OpenRcServicer;

impl ServiceBackend for OpenRcServicer {
    fn enable(&mut self, names: &[&str]) {
        for name in names {
            Command::new("sudo")
                .arg("rc-update")
                .arg("add")
                .arg(name)
                .arg("default")
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
    }

    fn disable(&mut self, _name: String) {
        todo!();

        // Command::new("sudo")
        //     .arg("rc-update")
        //     .arg("del")
        //     .arg(&name)
        //     .spawn()
        //     .unwrap()
        //     .wait()
        //     .unwrap();
    }

    fn list_all_enabled(&mut self) -> Vec<String> {
        todo!()
    }

    fn list_leaves_enabled(&mut self) -> Vec<String> {
        String::from_utf8(
            Command::new("sudo")
                .arg("rc-status")
                .arg("default")
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap()
        .lines()
        .skip(1)
        .map(|s| s.split('[').next().unwrap())
        .map(|s| s.trim())
        .map(|s| s.to_string())
        .collect()
    }
}
