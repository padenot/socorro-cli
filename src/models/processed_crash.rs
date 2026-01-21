use serde::{Deserialize, Serialize};
use super::StackFrame;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessedCrash {
    pub uuid: String,
    #[serde(default)]
    pub signature: Option<String>,
    #[serde(default)]
    pub product: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub os_name: Option<String>,
    #[serde(default)]
    pub os_version: Option<String>,

    #[serde(default)]
    pub crash_info: Option<CrashInfo>,
    #[serde(default)]
    pub moz_crash_reason: Option<String>,
    #[serde(default)]
    pub abort_message: Option<String>,

    #[serde(default)]
    pub android_model: Option<String>,
    #[serde(default)]
    pub android_version: Option<String>,

    #[serde(default)]
    pub crashing_thread: Option<usize>,
    #[serde(default)]
    pub threads: Option<Vec<Thread>>,
    #[serde(default)]
    pub json_dump: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrashInfo {
    #[serde(rename = "type")]
    pub crash_type: Option<String>,
    pub address: Option<String>,
    pub crashing_thread: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thread {
    pub thread: Option<usize>,
    pub thread_name: Option<String>,
    pub frames: Vec<StackFrame>,
}

#[derive(Debug, Clone)]
pub struct ThreadSummary {
    pub thread_index: usize,
    pub thread_name: Option<String>,
    pub frames: Vec<StackFrame>,
    pub is_crashing: bool,
}

#[derive(Debug)]
pub struct CrashSummary {
    pub crash_id: String,
    pub signature: String,
    pub reason: Option<String>,
    pub address: Option<String>,
    pub moz_crash_reason: Option<String>,
    pub abort_message: Option<String>,

    pub product: String,
    pub version: String,
    pub platform: String,

    pub android_version: Option<String>,
    pub android_model: Option<String>,

    pub crashing_thread_name: Option<String>,
    pub frames: Vec<StackFrame>,
    pub all_threads: Vec<ThreadSummary>,
}

impl ProcessedCrash {
    pub fn to_summary(&self, depth: usize, all_threads: bool) -> CrashSummary {
        let crashing_thread_idx = self.crashing_thread
            .or_else(|| self.crash_info.as_ref().and_then(|ci| ci.crashing_thread))
            .or_else(|| {
                self.json_dump.as_ref().and_then(|jd| {
                    jd.get("crashing_thread").and_then(|v| v.as_u64()).map(|v| v as usize)
                })
            });

        let json_dump_threads: Option<Vec<Thread>> = self.json_dump.as_ref()
            .and_then(|jd| jd.get("threads"))
            .and_then(|t| serde_json::from_value(t.clone()).ok());

        let threads_data = self.threads.as_ref()
            .or(json_dump_threads.as_ref());

        let (thread_name, frames, thread_summaries) = if let Some(threads) = threads_data {
            let mut all_thread_summaries = Vec::new();

            if all_threads {
                for (idx, thread) in threads.iter().enumerate() {
                    let frames: Vec<StackFrame> = thread.frames.iter()
                        .take(depth)
                        .cloned()
                        .collect();
                    all_thread_summaries.push(ThreadSummary {
                        thread_index: idx,
                        thread_name: thread.thread_name.clone(),
                        frames,
                        is_crashing: Some(idx) == crashing_thread_idx,
                    });
                }
            }

            if let Some(idx) = crashing_thread_idx {
                if let Some(thread) = threads.get(idx) {
                    let frames: Vec<StackFrame> = thread.frames.iter()
                        .take(depth)
                        .cloned()
                        .collect();
                    (thread.thread_name.clone(), frames, all_thread_summaries)
                } else {
                    (None, Vec::new(), all_thread_summaries)
                }
            } else {
                (None, Vec::new(), all_thread_summaries)
            }
        } else {
            (None, Vec::new(), Vec::new())
        };

        let json_dump_crash_info: Option<CrashInfo> = self.json_dump.as_ref()
            .and_then(|jd| jd.get("crash_info"))
            .and_then(|ci| serde_json::from_value(ci.clone()).ok());

        let crash_info = self.crash_info.as_ref()
            .or(json_dump_crash_info.as_ref());

        CrashSummary {
            crash_id: self.uuid.clone(),
            signature: self.signature.clone().unwrap_or_else(|| "Unknown".to_string()),
            reason: crash_info.and_then(|ci| ci.crash_type.clone()),
            address: crash_info.and_then(|ci| ci.address.clone()),
            moz_crash_reason: self.moz_crash_reason.clone(),
            abort_message: self.abort_message.clone(),
            product: self.product.clone().unwrap_or_else(|| "Unknown".to_string()),
            version: self.version.clone().unwrap_or_else(|| "Unknown".to_string()),
            platform: format!(
                "{}{}",
                self.os_name.as_deref().unwrap_or("Unknown"),
                self.os_version.as_ref().map(|v| format!(" {}", v)).unwrap_or_default()
            ),
            android_version: self.android_version.clone(),
            android_model: self.android_model.clone(),
            crashing_thread_name: thread_name,
            frames,
            all_threads: thread_summaries,
        }
    }
}
