# tauri-plugin-cron

[![crates.io](https://img.shields.io/crates/v/tauri-plugin-cron)](https://crates.io/crates/tauri-plugin-cron)
[![npm](https://img.shields.io/npm/v/tauri-plugin-cron)](https://www.npmjs.com/package/tauri-plugin-cron)

> [English](README_EN.md)

一个 Tauri 2.0 插件，提供基于 cron 表达式的定时任务调度能力。任务在 Rust 后台线程运行，通过事件通知前端执行回调。

---

## 安装

### Rust（`src-tauri/Cargo.toml`）

```toml
[dependencies]
tauri-plugin-cron = "0.1"
```

### npm（`package.json`）

```json
{
  "dependencies": {
    "tauri-plugin-cron": "^0.1.0"
  }
}
```

### 权限（`src-tauri/capabilities/default.json`）

```json
{
  "permissions": ["cron:default"]
}
```

---

## 使用

### Rust 注册插件（`src-tauri/src/lib.rs`）

```rust
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_cron::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 前端调用

```typescript
import { addCronJob, removeCronJobByName } from 'tauri-plugin-cron'

// 每 5 秒执行一次
await addCronJob('my-job', '*/5 * * * * *', (jobInfo) => {
  console.log('cron triggered:', jobInfo.name)
})

// 按名称移除
await removeCronJobByName('my-job')
```

---

## API

### `addCronJob(name, cronExpression, executor)`

添加一个定时任务。

| 参数 | 类型 | 说明 |
|---|---|---|
| `name` | `string` | 任务唯一标识 |
| `cronExpression` | `string` | Cron 表达式，格式为 `sec min hour day month week [year]` |
| `executor` | `(jobInfo: JobInfo) => void \| Promise<void>` | 触发时执行的回调 |

**返回值**: `Promise<JobInfo>`

### `removeCronJob(id)`

按 ID 移除任务。

| 参数 | 类型 | 说明 |
|---|---|---|
| `id` | `string` | 任务 ID |

### `removeCronJobByName(name)`

按名称移除任务。

| 参数 | 类型 | 说明 |
|---|---|---|
| `name` | `string` | 任务名称 |

### `listCronJobs()`

列出所有活跃任务。

**返回值**: `Promise<JobInfo[]>`

### `JobInfo`

```typescript
interface JobInfo {
  id: string
  name: string
  cron_expression: string
}
```

---

## Cron 表达式格式

6 或 7 个空格分隔的字段：

```
sec min hour day month week [year]
```

| 字段 | 范围 | 说明 |
|---|---|---|
| `sec` | 0-59 | 秒 |
| `min` | 0-59 | 分 |
| `hour` | 0-23 | 时 |
| `day` | 1-31 | 日 |
| `month` | 1-12 | 月 |
| `week` | 0-7 | 周（0 和 7 都表示周日） |
| `year` | 可选 | 年 |

支持的运算符：`*`（所有值）、`*/n`（每 n 个单位）、`,`（列表）、`-`（范围）。

### 示例

| 表达式 | 说明 |
|---|---|
| `*/5 * * * * *` | 每 5 秒 |
| `0 */2 * * * *` | 每 2 分钟 |
| `0 30 9 * * *` | 每天 09:30:00 |
| `0 0 0 * * 1-5` | 工作日 00:00:00 |
| `0 0 0 1 1 *` | 每年 1 月 1 日 |

---

## 构建

```bash
cd tauri-plugin-cron

# 编译 Rust
cargo build

# TypeScript 类型检查
npx vue-tsc --noEmit
```

---

## License

MIT
