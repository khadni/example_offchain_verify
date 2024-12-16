use example_verify::verify_report;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <program-id> <access-controller> <hex-encoded-signed-report>", args[0]);
        std::process::exit(1);
    }

    let program_id = &args[1];
    let access_controller = &args[2];
    let hex_report = &args[3];

    // Decode the hex string
    let signed_report = match hex::decode(hex_report) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Failed to decode hex string: {}", e);
            std::process::exit(1);
        }
    };

    // Verify the report
    match verify_report(&signed_report, program_id, access_controller) {
        Ok(report) => {
            println!("FeedId: {}", report.feed_id);
            println!("valid from timestamp: {}", report.valid_from_timestamp);
            println!("observations timestamp: {}", report.observations_timestamp);
            println!("native fee: {}", report.native_fee);
            println!("link fee: {}", report.link_fee);
            println!("expires at: {}", report.expires_at);
            println!("benchmark_price: {}", report.benchmark_price);
            println!("bid: {}", report.bid);
            println!("ask: {}", report.ask);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
} 