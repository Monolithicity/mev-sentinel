use std::path::PathBuf;

struct Opts {
    help: bool,

    reset: bool,

    overwrite: bool,

    url: String,

    cache: Option<PathBuf>,


}



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let opts = Opts::parse_args_default_or_exit();
}
