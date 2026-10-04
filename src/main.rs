pub mod tasks;
pub mod msgs;

use cu29::prelude::{
    MatchingTasks,
    ErasedCuStampedData,
    ErasedCuStampedDataSet,
    CuListZeroedInit,
    ANONYMOUS,
    CuLogLevel,
    CuLogEntry,
    CuCompactString,
    error,
    log_debug_mode,
    ctrlc,
    to_value
};

const PREALLOCATED_STORAGE_SIZE: Option<usize> = Some(1024 * 1024 * 100);
#[cu29::prelude::copper_runtime(config = "configs.ron")]
struct AstarAlgoApplication {}


fn main() {

    let logger_path = "logs/astar-algo.copper";
    
    if let Some(parent) = std::path::Path::new(logger_path).parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).expect("Failed to create logs directory");
        }
    }

    println!("Logger created at {}.", logger_path);
    println!("Creating application... ");

    let application = AstarAlgoApplication::builder()
        .with_log_path(logger_path, PREALLOCATED_STORAGE_SIZE)
        .expect("Failed to setup logger.")
        .build()
        .expect("Failed to create application.");
    // println!("Running... starting clock: {}.", application.clock().now());

    let stopped = application.run_until_shutdown().expect("Failed to run application.");
    // println!("End of program: {}.", stopped.clock().now());
    // std::thread::sleep(Dstd::time::Duration::from_secs(1));        

}
