mod commands;
mod models;

use commands::CronState;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("cron")
        .setup(|_app, _api| {
            _app.manage(CronState::new());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::add_cron_job,
            commands::remove_cron_job,
            commands::list_cron_jobs,
        ])
        .build()
}
