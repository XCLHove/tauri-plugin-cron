import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'

export interface JobInfo {
  id: string
  name: string
  cron_expression: string
}

export type CronJobExecutor = (jobInfo: JobInfo) => void | Promise<void>

const jobInfoByName = new Map<string, JobInfo>()
const executorById = new Map<string, CronJobExecutor>()

let unlistenInitialized = false
let unlistenFn: (() => void) | null = null

function ensureListener(): Promise<void> {
  if (unlistenInitialized) return Promise.resolve()
  unlistenInitialized = true

  return getCurrentWindow()
    .listen<JobInfo>('cron-job-triggered', ({ payload: jobInfo }) => {
      const cachedJobInfo = jobInfoByName.get(jobInfo.name)

      // If the ID doesn't match, the job was replaced
      if (jobInfo.id !== cachedJobInfo?.id) {
        removeCronJob(jobInfo.id).catch(() => {})
        return
      }

      const executor = executorById.get(jobInfo.id)
      if (!executor) {
        removeCronJob(jobInfo.id).catch(() => {})
        return
      }

      Promise.resolve().then(() => executor(jobInfo))
    })
    .then((unlisten) => {
      unlistenFn = unlisten
    })
}

window.addEventListener('unload', () => {
  unlistenFn?.()
})

/**
 * Add a cron job.
 *
 * @param name - A unique name for the cron job
 * @param cronExpression - A cron expression
 * @param executor - Callback function executed when the cron job triggers
 * @returns The created job info
 */
export async function addCronJob(
  name: string,
  cronExpression: string,
  executor: CronJobExecutor
): Promise<JobInfo> {
  await ensureListener()
  const jobInfo = await invoke<JobInfo>('plugin:cron|add_cron_job', {
    name,
    cronExpression,
  })
  jobInfoByName.set(jobInfo.name, jobInfo)
  executorById.set(jobInfo.id, executor)
  return { ...jobInfo }
}

/**
 * Remove a cron job by its ID.
 *
 * @param id - The ID of the cron job to remove
 */
export async function removeCronJob(id: string): Promise<void> {
  await invoke<void>('plugin:cron|remove_cron_job', { id })
  executorById.delete(id)
  // Clean up name map
  for (const [name, info] of jobInfoByName.entries()) {
    if (info.id === id) {
      jobInfoByName.delete(name)
      break
    }
  }
}

/**
 * Remove a cron job by its name.
 *
 * @param name - The name of the cron job to remove
 */
export async function removeCronJobByName(name: string): Promise<void> {
  const jobInfo = jobInfoByName.get(name)
  if (!jobInfo) return
  await removeCronJob(jobInfo.id)
}

/**
 * List all active cron jobs.
 *
 * @returns An array of all active cron jobs
 */
export async function listCronJobs(): Promise<JobInfo[]> {
  return invoke<JobInfo[]>('plugin:cron|list_cron_jobs')
}
