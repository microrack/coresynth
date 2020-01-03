use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(all(target_arch="arm", target_os="none"))] {
        use crate::num::Round;
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum JogDirection {
    Left,
    Right,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Buttons {
    S(usize),
    Shift,
    Play,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GlobalEvent {
	Jog(JogDirection),
	
    Wakeup,
    Info,
}
