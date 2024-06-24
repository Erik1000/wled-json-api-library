
use crate::structures::cfg::Cfg;
use crate::structures::effects::Effects;
use crate::structures::info::Info;
use crate::structures::live::Live;
use crate::structures::net::Net;
use crate::structures::nodes::Nodes;
use crate::structures::palettes::Palettes;
use crate::structures::state::State;

#[derive(Debug)]
pub struct Wled {
    pub effects: Option<Effects>,
    pub palettes: Option<Palettes>,
    pub state: Option<State>,
    pub info: Option<Info>,
    pub cfg: Option<Cfg>,
    pub live: Option<Live>,
    pub nodes: Option<Nodes>,
    pub net: Option<Net>,
}
