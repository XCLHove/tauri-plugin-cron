use crate::models::JobInfo;
use chrono::Local;
use cron_tab::Cron;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{Emitter, Manager, Runtime, Window};
use uuid::Uuid;

pub struct CronState {
    cron: Mutex<Option<Cron<Local>>>,
    job_info_by_id: Mutex<HashMap<String, JobInfo>>,
    cron_job_id_by_info_id: Mutex<HashMap<String, usize>>,
}

impl CronState {
    pub fn new() -> Self {
        Self {
            cron: Mutex::new(None),
            job_info_by_id: Mutex::new(HashMap::new()),
            cron_job_id_by_info_id: Mutex::new(HashMap::new()),
        }
    }

    fn generate_job_id() -> String {
        Uuid::new_v4().to_string()
    }

    fn get_or_create_cron(&self) -> Result<Cron<Local>, String> {
        let mut cron_guard = self.cron.lock().map_err(|e| e.to_string())?;
        if let Some(cron) = cron_guard.as_ref() {
            return Ok(cron.clone());
        }
        let mut cron = Cron::new(Local);
        cron.start();
        let cron_clone = cron.clone();
        *cron_guard = Some(cron);
        Ok(cron_clone)
    }
}

#[tauri::command]
pub fn add_cron_job<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: Window<R>,
    state: tauri::State<'_, CronState>,
    name: String,
    cron_expression: String,
) -> Result<JobInfo, String> {
    let mut cron = state.get_or_create_cron()?;
    let job_info_id = CronState::generate_job_id();
    let job_info_id_for_closure = job_info_id.clone();
    let window_label = window.label().to_string();
    let job_info_name = name.clone();
    let cron_expression_for_closure = cron_expression.clone();

    let cron_job_id = cron
        .add_fn(&cron_expression, move || {
            if let Some(window) = app.get_webview_window(&window_label) {
                let _ = window.emit(
                    "cron-job-triggered",
                    JobInfo {
                        id: job_info_id_for_closure.clone(),
                        name: job_info_name.clone(),
                        cron_expression: cron_expression_for_closure.clone(),
                    },
                );
            }
        })
        .map_err(|e| e.to_string())?;

    state
        .cron_job_id_by_info_id
        .lock()
        .map_err(|e| e.to_string())?
        .insert(job_info_id.clone(), cron_job_id);

    let job_info = JobInfo {
        id: job_info_id,
        name,
        cron_expression,
    };

    state
        .job_info_by_id
        .lock()
        .map_err(|e| e.to_string())?
        .insert(job_info.id.clone(), job_info.clone());

    Ok(job_info)
}

#[tauri::command]
pub fn remove_cron_job(state: tauri::State<'_, CronState>, id: String) -> Result<(), String> {
    state
        .job_info_by_id
        .lock()
        .map_err(|e| e.to_string())?
        .remove(&id);

    let cron_job_id = state
        .cron_job_id_by_info_id
        .lock()
        .map_err(|e| e.to_string())?
        .remove(&id);

    if let Some(cron_job_id) = cron_job_id {
        let mut cron_guard = state.cron.lock().map_err(|e| e.to_string())?;
        if let Some(cron) = cron_guard.as_mut() {
            cron.remove(cron_job_id);
        }
    }

    Ok(())
}

#[tauri::command]
pub fn list_cron_jobs(state: tauri::State<'_, CronState>) -> Result<Vec<JobInfo>, String> {
    let jobs = state
        .job_info_by_id
        .lock()
        .map_err(|e| e.to_string())?;
    Ok(jobs.values().cloned().collect())
}
