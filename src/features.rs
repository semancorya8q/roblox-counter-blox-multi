pub struct Features {
    aimbot_enabled: bool,
    esp_enabled: bool,
    speed_hack_enabled: bool,
}

impl Features {
    pub fn new() -> Self {
        Features {
            aimbot_enabled: false,
            esp_enabled: false,
            speed_hack_enabled: false,
        }
    }

    pub fn toggle_aimbot(&mut self) {
        self.aimbot_enabled = !self.aimbot_enabled;
    }

    pub fn toggle_esp(&mut self) {
        self.esp_enabled = !self.esp_enabled;
    }

    pub fn toggle_speed_hack(&mut self) {
        self.speed_hack_enabled = !self.speed_hack_enabled;
    }
}