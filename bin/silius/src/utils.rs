use dirs::home_dir;
use ethers::{
    providers::{Http, Provider, Ws},
    types::{Address, U256},
};
use expanded_pathbuf::ExpandedPathBuf;
use pin_utils::pin_mut;
use silius_primitives::{bundler::SendBundleMode, UoPoolMode};
use std::{future::Future, str::FromStr};
use tracing::info;

/// Unwrap path or returns home directory
pub fn unwrap_path_or_home(path: Option<ExpandedPathBuf>) -> eyre::Result<ExpandedPathBuf> {
    if let Some(path) = path {
        Ok(path)
    } else {
        home_dir()
            .map(|h| h.join(".silius"))
            .ok_or_else(|| eyre::eyre!("Get Home directory error"))
            .map(ExpandedPathBuf)
    }
}

/// Parses address from string
pub fn parse_address(s: &str) -> Result<Address, String> {
    Address::from_str(s).map_err(|_| format!("String {s} is not a valid address"))
}

/// Parses U256 from string
pub fn parse_u256(s: &str) -> Result<U256, String> {
    U256::from_str_radix(s, 10).map_err(|_| format!("String {s} is not a valid U256"))
}

/// Parses SendBundleMode from string
pub fn parse_send_bundle_mode(s: &str) -> Result<SendBundleMode, String> {
    SendBundleMode::from_str(s).map_err(|_| format!("String {s} is not a valid SendBundleMode"))
}

/// Parses UoPoolMode from string
pub fn parse_uopool_mode(s: &str) -> Result<UoPoolMode, String> {
    UoPoolMode::from_str(s).map_err(|_| format!("String {s} is not a valid UoPoolMode"))
}

/// Runs the future to completion or until:
/// - `ctrl-c` is received.
/// - `SIGTERM` is received (unix only).
pub async fn run_until_ctrl_c<F, E>(fut: F) -> Result<(), E>
where
    F: Future<Output = Result<(), E>>,
    E: Send + Sync + 'static + From<std::io::Error>,
{
    let ctrl_c = tokio::signal::ctrl_c();

    let mut stream = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    let sigterm = stream.recv();
    pin_mut!(sigterm, ctrl_c, fut);

    tokio::select! {
        _ = ctrl_c => {
            info!("Received ctrl-c signal.");
        },
        _ = sigterm => {
            info!("Received SIGTERM signal.");
        },
        res = fut => res?,
    }

    Ok(())
}

/// Creates ethers provider with HTTP connection
pub fn create_http_provider(addr: &str) -> eyre::Result<Provider<Http>> {
    let provider = Provider::<Http>::try_from(addr)?;
    Ok(provider)
}

/// Creates ethers provider with WebSockets connection
pub async fn create_ws_provider(addr: &str) -> eyre::Result<Provider<Ws>> {
    let provider = Provider::<Ws>::connect_with_reconnects(addr, usize::MAX).await?;
    Ok(provider)
}
