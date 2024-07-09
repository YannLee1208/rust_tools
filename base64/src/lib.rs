mod cli;
mod process;
mod utils;

pub use cli::{Args, B64Command, Base64Format};
pub use process::{process_decode, process_encode};
