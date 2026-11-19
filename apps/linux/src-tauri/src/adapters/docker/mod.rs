//! Adapter docker: satu pintu CLI agar service bisa dites tanpa proses nyata.

mod cli;
mod parse;

pub use cli::{DockerCli, SystemDockerCli, MAX_OUTPUT_BYTES};
pub use parse::{
    mark_in_use, parse_containers, parse_df, parse_df_used_bytes, parse_images, parse_size,
};
