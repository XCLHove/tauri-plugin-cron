const COMMANDS: &[&str] = &["add_cron_job", "remove_cron_job", "list_cron_jobs"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
