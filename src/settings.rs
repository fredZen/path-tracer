use super::Settings;

pub fn low() -> Settings {
    Settings {
        width: 200,
        height: 100,
        samples: 100,
        depth: 50,
    }
}

pub fn high() -> Settings {
    Settings {
        width: 1280,
        height: 720,
        samples: 100,
        depth: 50,
    }
}
