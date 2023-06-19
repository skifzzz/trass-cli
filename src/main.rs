mod cli;

use anyhow::anyhow;
use cli::Arguments;
use trass_core::config::Config;
use trass_core::data_access::DataAccessService;
use log::error;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let arguments = Arguments::parse_or_default();
    let config_dir = arguments.config_dir.ok_or(anyhow!("Can't read config"))?;

    let config = Config::init(config_dir)?;

    let data_access_service = DataAccessService::new(&config).await;
    data_access_service
        .import_order_history_from(arguments.data_dir.as_ref().expect("Can't find data dir"))
        .await;

    match data_access_service.get_recommendations().await {
        Ok(recommendations) => {
            println!("{}", recommendations)
        }
        Err(e) => error!("No recommendations available, {:?}", e),
    }

    Ok(())
}