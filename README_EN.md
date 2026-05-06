# tauri-plugin-cron

[![crates.io](https://img.shields.io/crates/v/tauri-plugin-cron)](https://crates.io/crates/tauri-plugin-cron)
[![npm](https://img.shields.io/npm/v/tauri-plugin-cron)](https://www.npmjs.com/package/tauri-plugin-cron)

> [中文](README.md)

A Tauri 2.0 plugin for cron-based job scheduling. Jobs run on Rust backend threads and notify the frontend via events.

---

## Installation

### Rust

```toml
# src-tauri/Cargo.toml
[dependencies]
tauri-plugin-cron = "0.1"
```

Or via CLI:

```bash
cd src-tauri
cargo add tauri-plugin-cron
```

### Frontend

```bash
# npm
npm install tauri-plugin-cron

# pnpm
pnpm add tauri-plugin-cron
```

### Permissions (`src-tauri/capabilities/default.json`)

```json
{
  "permissions": ["cron:default"]
}
```

---

## Usage

### Register the plugin (`src-tauri/src/lib.rs`)

```rust
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_cron::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Frontend API

```typescript
import { addCronJob, removeCronJobByName } from 'tauri-plugin-cron'

// Run every 5 seconds
await addCronJob('my-job', '*/5 * * * * *', (jobInfo) => {
  console.log('cron triggered:', jobInfo.name)
})

// Remove by name
await removeCronJobByName('my-job')
```

---

## API

### `addCronJob(name, cronExpression, executor)`

Add a cron job.

| Param | Type | Description |
|---|---|---|
| `name` | `string` | Unique job name |
| `cronExpression` | `string` | Cron expression in `sec min hour day month week [year]` format |
| `executor` | `(jobInfo: JobInfo) => void \| Promise<void>` | Callback executed on trigger |

**Returns**: `Promise<JobInfo>`

### `removeCronJob(id)`

Remove a cron job by ID.

| Param | Type | Description |
|---|---|---|
| `id` | `string` | Job ID |

### `removeCronJobByName(name)`

Remove a cron job by name.

| Param | Type | Description |
|---|---|---|
| `name` | `string` | Job name |

### `listCronJobs()`

List all active cron jobs.

**Returns**: `Promise<JobInfo[]>`

### `JobInfo`

```typescript
interface JobInfo {
  id: string
  name: string
  cron_expression: string
}
```

---

## Cron Expression Format

6 or 7 space-separated fields:

```
sec min hour day month week [year]
```

| Field | Range | Description |
|---|---|---|
| `sec` | 0-59 | Seconds |
| `min` | 0-59 | Minutes |
| `hour` | 0-23 | Hours |
| `day` | 1-31 | Day of month |
| `month` | 1-12 | Month |
| `week` | 0-7 | Day of week (0/7 = Sun) |
| `year` | Optional | Year |

Supported operators: `*` (all values), `*/n` (every n units), `,` (list), `-` (range).

### Examples

| Expression | Description |
|---|---|
| `*/5 * * * * *` | Every 5 seconds |
| `0 */2 * * * *` | Every 2 minutes |
| `0 30 9 * * *` | Daily at 09:30:00 |
| `0 0 0 * * 1-5` | Weekdays at midnight |
| `0 0 0 1 1 *` | Every Jan 1 |

---

## Build

```bash
cd tauri-plugin-cron

# Build Rust
cargo build

# TypeScript type check
npx vue-tsc --noEmit
```

---

## License

MIT
