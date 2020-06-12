pub use self::{
    dude::{init_dude, Dude, DudeState},
    subject::Subject,
};

mod dude;

pub mod animation;
mod subject;
pub mod velocity;