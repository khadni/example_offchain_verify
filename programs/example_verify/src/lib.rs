use data_streams_report::report::v3::ReportDataV3;
use sdk_off_chain::VerificationClient;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::read_keypair_file,
    signer::Signer,
};
use std::path::PathBuf;
use std::str::FromStr;

pub fn default_keypair_path() -> String {
    let mut path = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()));
    path.push(".config/solana/id.json");
    path.to_str().unwrap().to_string()
}

pub fn verify_report(signed_report: &[u8], program_id: &str, access_controller: &str) -> Result<ReportDataV3, Box<dyn std::error::Error>> {
    // Initialize RPC client with confirmed commitment level
    let rpc_client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com",
        CommitmentConfig::confirmed()
    );

    // Load the keypair that will pay for and sign verification transactions
    let payer = read_keypair_file(default_keypair_path())?;
    println!("Using keypair: {}", payer.try_pubkey()?);

    // Set verifier program and access controller addresses
    let program_id = Pubkey::from_str(program_id)?;
    let access_controller = Pubkey::from_str(access_controller)?;
    println!("Program ID: {}", program_id);
    println!("Access Controller: {}", access_controller);

    // Create verification client
    let client = VerificationClient::new(program_id, access_controller, rpc_client, payer);

    // Verify the report
    println!("Verifying report of {} bytes...", signed_report.len());
    let result = client.verify(signed_report.to_vec()).map_err(|e| {
        println!("Verification error: {:?}", e);
        e
    })?;
    
    let report = ReportDataV3::decode(&result.return_data.ok_or("No return data")?)?;
    Ok(report)
} 