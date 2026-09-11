use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use crate::models::{InitPayload, ScreenshotResult};

pub struct ActiveSession {
    pub job_id: String,
    pub init_payload: InitPayload,
    pub result_sender: Option<oneshot::Sender<ScreenshotResult>>,
    pub last_output_bytes: Option<Vec<u8>>,
    pub frame_png_bytes: Option<Vec<u8>>,
}

#[derive(Clone, Default)]
pub struct SessionManager {
    inner: Arc<Mutex<Option<ActiveSession>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(None)),
        }
    }

    /// 存储全屏截取的 PNG 图片字节，供前端直接通过二进制 IPC 极速获取（避免 JSON 传输几兆 base64）
    pub fn store_frame_bytes(&self, bytes: Vec<u8>) {
        let mut guard = self.inner.lock().unwrap();
        if let Some(ref mut session) = *guard {
            session.frame_png_bytes = Some(bytes);
        }
    }

    /// 获取缓存的全屏图片字节
    pub fn get_frame_bytes(&self) -> Option<Vec<u8>> {
        let guard = self.inner.lock().unwrap();
        guard.as_ref().and_then(|s| s.frame_png_bytes.clone())
    }

    /// 存储 output 生成的图片字节，避免 confirm 重复序列化传输
    pub fn store_output_bytes(&self, bytes: Vec<u8>) {
        let mut guard = self.inner.lock().unwrap();
        if let Some(ref mut session) = *guard {
            session.last_output_bytes = Some(bytes);
        }
    }

    /// 获取缓存的图片字节
    pub fn get_output_bytes(&self) -> Option<Vec<u8>> {
        let guard = self.inner.lock().unwrap();
        guard.as_ref().and_then(|s| s.last_output_bytes.clone())
    }

    /// 启动新会话，若存在旧会话则通知取消
    pub fn start_session(
        &self,
        job_id: String,
        init_payload: InitPayload,
        sender: oneshot::Sender<ScreenshotResult>,
    ) {
        let mut guard = self.inner.lock().unwrap();
        if let Some(mut prev) = guard.take() {
            crate::logger::write_log("Snapora:Session", &format!("⚠️ 发现未完成旧会话 [{}]，被新会话 [{}] 强制取消！", prev.job_id, job_id));
            if let Some(s) = prev.result_sender.take() {
                let _ = s.send(ScreenshotResult::Cancelled);
            }
        }
        crate::logger::write_log("Snapora:Session", &format!("注册新会话 [{job_id}]"));
        *guard = Some(ActiveSession {
            job_id,
            init_payload,
            result_sender: Some(sender),
            last_output_bytes: None,
            frame_png_bytes: None,
        });
    }

    /// 获取当前会话的初始化 Payload（供 overlay_ready 投递）
    pub fn get_init_payload(&self) -> Option<InitPayload> {
        let guard = self.inner.lock().unwrap();
        guard.as_ref().map(|s| s.init_payload.clone())
    }

    /// 完成当前会话
    pub fn complete_session(&self, result: ScreenshotResult) {
        let mut guard = self.inner.lock().unwrap();
        if let Some(mut session) = guard.take() {
            let summary = match &result {
                ScreenshotResult::Completed { data, bounds, output, .. } => {
                    format!("status: completed, bytes: {}, bounds: {bounds:?}, output: {output:?}", data.len())
                }
                ScreenshotResult::Cancelled => "status: cancelled".to_string(),
                ScreenshotResult::Failed { code, message } => {
                    format!("status: failed, code: {code}, message: {message}")
                }
            };
            crate::logger::write_log("Snapora:Session", &format!("会话 [{}] 正常完成 ({summary})", session.job_id));
            if let Some(sender) = session.result_sender.take() {
                let _ = sender.send(result);
            }
        } else {
            crate::logger::write_log("Snapora:Session", "警告：complete_session 调用时未找到活跃会话！");
        }
    }

    /// 取消当前会话
    pub fn cancel_session(&self) {
        let mut guard = self.inner.lock().unwrap();
        if let Some(mut session) = guard.take() {
            crate::logger::write_log("Snapora:Session", &format!("会话 [{}] 被 cancel_session() 显式取消", session.job_id));
            if let Some(sender) = session.result_sender.take() {
                let _ = sender.send(ScreenshotResult::Cancelled);
            }
        } else {
            crate::logger::write_log("Snapora:Session", "提示：cancel_session 调用时无活跃会话");
        }
    }
}
