use nexus::data_link::mumble::MumblePtr;
use nexus::data_link::rtapi::read_rtapi;
use nexus::data_link::{get_mumble_link, get_nexus_link, NexusLink};
use nexus::rtapi::data::RealTimeData;

#[derive(Debug, Clone)]
pub struct Links {
    pub mumble: Option<MumblePtr>,
    pub rtapi: Option<RealTimeData>,
    nexus: *const NexusLink,
}

impl Default for Links {
    fn default() -> Self {
        let nexus = get_nexus_link();
        if nexus.is_null() {
            log::error!("Failed to get Nexus link")
        }
        Self {
            mumble: get_mumble_link(),
            nexus,
            rtapi: None,
        }
    }
}

impl Links {
    pub unsafe fn nexus(&self) -> Option<&NexusLink> {
        self.nexus.as_ref()
    }
    pub unsafe fn update_rtapi(&mut self) {
        if let Some(rtapi) = read_rtapi() {
            if rtapi.game_build != 0 {
                self.rtapi = Some(rtapi);
            } else {
                self.rtapi = None
            }
        }
    }
}

unsafe impl Send for Links {}
unsafe impl Sync for Links {}
