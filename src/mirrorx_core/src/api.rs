use crate::{api_error::APIError, service::flutter_command::FlutterCommand};
use env_logger::{Builder, Target};
use flutter_rust_bridge::*;
use log::LevelFilter;
use std::{io::Write, path::PathBuf, sync::Once};

static INIT_ONCE: Once = Once::new();
static mut INIT_ONCE_RESULT: anyhow::Result<(), APIError> = Ok(());

pub fn init(config_db_path: String) -> anyhow::Result<()> {
    unsafe {
        INIT_ONCE.call_once(|| {
            Builder::new()
                .filter_level(LevelFilter::Info)
                .format(|buf, record| {
                    writeln!(
                        buf,
                        "[{}] [{}({})] {} {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
                        record.module_path().unwrap_or(""),
                        record.file().unwrap_or(""),
                        record.level(),
                        record.args()
                    )
                })
                .target(Target::Stdout)
                .init();

            let init_fn = || -> anyhow::Result<(), APIError> {
                let config_db_path = PathBuf::from(config_db_path);

                crate::service::config::init_config(config_db_path)?;
                crate::service::runtime::init_async_runtime()?;
                crate::service::network::init_client()?;

                Ok(())
            };

            INIT_ONCE_RESULT = init_fn();
        });

        match &INIT_ONCE_RESULT {
            Ok(_) => Ok(()),
            Err(err) => Err(anyhow::anyhow!(err)),
        }
    }
}

pub fn init_flutter_command_stream_sink(
    flutter_command_stream_sink: StreamSink<FlutterCommand>,
) -> anyhow::Result<()> {
    crate::service::flutter_command::init_flutter_command_stream_sink(flutter_command_stream_sink);
    Ok(())
}

pub fn read_device_id() -> anyhow::Result<Option<String>> {
    crate::service::config::read_device_id().map_err(|err| anyhow::anyhow!(err))
}

pub fn read_device_password() -> anyhow::Result<Option<String>> {
    crate::service::config::read_device_password().map_err(|err| anyhow::anyhow!(err))
}

pub fn save_device_password(device_password: String) -> anyhow::Result<()> {
    crate::service::config::save_device_password(&device_password)
        .map_err(|err| anyhow::anyhow!(err))
}

pub fn generate_random_device_password() -> anyhow::Result<String> {
    let password = crate::service::utility::generate_device_password();
    crate::service::config::save_device_password(&password)
        .map(|_| password)
        .map_err(|err| anyhow::anyhow!(err))
}

pub fn device_goes_online() -> anyhow::Result<()> {
    crate::service::network::device_goes_online().map_err(|err| anyhow::anyhow!(err))
}

pub fn desktop_connect_offer(ask_device_id: String) -> anyhow::Result<bool> {
    crate::service::network::desktop_connect_offer(ask_device_id)
        .map_err(|err| anyhow::anyhow!(err))
}

pub fn desktop_connect_offer_auth_password(
    ask_device_id: String,
    device_password: String,
) -> anyhow::Result<bool> {
    crate::service::network::desktop_connect_offer_auth_password(ask_device_id, device_password)
        .map_err(|err| anyhow::anyhow!(err))
}

pub fn desktop_connect_open_stream(ask_device_id: String) -> anyhow::Result<()> {
    crate::service::network::desktop_connect_open_stream(ask_device_id)
        .map_err(|err| anyhow::anyhow!(err))
}
