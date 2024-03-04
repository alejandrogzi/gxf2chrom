use log::{error, info, warn, Level};
use simple_logger;

use gxf2chrom::{
    cli::Args,
    utils::{max_mem_usage_mb, parallel_parse, reader, write_obj},
};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(Level::Info).unwrap();
    info!("{} v{}", PKG_NAME, PKG_VERSION);
    warn!("For any bug/issue contact: {}", PKG_REPOSITORY);

    let args: Args = Args::get();
    args.check().unwrap_or_else(|e| {
        error!("{}", e);
        std::process::exit(1);
    });
    info!("{:?}", args);

    rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()
        .unwrap();

    let st = std::time::Instant::now();
    let start_mem = max_mem_usage_mb();

    let gxf = reader(&args.gxf)?;
    let contents = parallel_parse(&gxf, &args.feature)?;
    write_obj(&args.output, &contents)?;

    let elapsed = st.elapsed();
    let mem = (max_mem_usage_mb() - start_mem).max(0.0);

    info!("Elapsed: {:.4?} secs", elapsed.as_secs_f32());
    info!("Memory: {:.2} MB", mem);
    info!("Thank you for using {}!", PKG_NAME);

    Ok(())
}
