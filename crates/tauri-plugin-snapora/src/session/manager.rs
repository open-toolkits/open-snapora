use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use crate::error::Error;
use crate::models::{ImageRef, InitPayload, ScreenshotResult, SessionStatus};

/// 描述一个活跃截图会话的状态与上下文，不直接持有庞大的图像像素数组
pub struct CaptureSession {
    pub job_id: String,
    pub status: SessionStatus,
    pub init_payload: InitPayload,
    pub source_image_ref: Option<ImageRef>,
    pub result_sender: Option<oneshot::Sender<ScreenshotResult>>,
}

/// 集中式会话管理器（SessionManager）
/// 
/// 负责管理全生命周期的 CaptureSession。
/// 严格执行 Job ID 校验，杜绝旧 Session 的异步回调覆盖或影响当前 Session。
#[derive(Clone, Default)]
pub struct SessionManager {
    inner: Arc<Mutex<Option<CaptureSession>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(None)),
        }
    }

    /// 获取当前活跃会话的 Job ID
    pub fn current_job_id(&self) -> Option<String> {
        let guard = self.inner.lock().unwrap();
        guard.as_ref().map(|s| s.job_id.clone())
    }

    /// 获取当前会话的状态
    pub fn get_status(&self, job_id: Option<&str>) -> Option<SessionStatus> {
        let guard = self.inner.lock().unwrap();
        if let Some(ref session) = *guard {
            if let Some(jid) = job_id {
                if session.job_id != jid {
                    return None;
                }
            }
            return Some(session.status);
        }
        None
    }

    /// 启动新会话；若存在未完成的旧会话，向其通道发送 Cancelled 并予以顶替
    pub fn start_session(
        &self,
        job_id: String,
        init_payload: InitPayload,
        source_image_ref: Option<ImageRef>,
        sender: oneshot::Sender<ScreenshotResult>,
    ) {
        let mut guard = self.inner.lock().unwrap();
        if let Some(mut prev) = guard.take() {
            crate::logger::write_log(
                "Snapora:Session",
                &format!("⚠️ 发现未完成旧会话 [{}]，被新会话 [{}] 强制取代！", prev.job_id, job_id),
            );
            if let Some(s) = prev.result_sender.take() {
                let _ = s.send(ScreenshotResult::Cancelled);
            }
        }
        crate::logger::write_log("Snapora:Session", &format!("注册新会话 [{job_id}]，状态: Created"));
        *guard = Some(CaptureSession {
            job_id,
            status: SessionStatus::Created,
            init_payload,
            source_image_ref,
            result_sender: Some(sender),
        });
    }

    /// 更新会话状态机，必须校验 job_id 匹配
    pub fn update_status(&self, job_id: &str, new_status: SessionStatus) -> Result<(), Error> {
        let mut guard = self.inner.lock().unwrap();
        if let Some(ref mut session) = *guard {
            if session.job_id != job_id {
                crate::logger::write_log(
                    "Snapora:Session",
                    &format!("⚠️ update_status 拒绝: 传入的 job_id [{job_id}] 与当前活跃 [{}] 不匹配", session.job_id),
                );
                return Err(Error::StaleSession(format!(
                    "请求的任务 [{job_id}] 已过期，当前活跃任务为 [{}]",
                    session.job_id
                )));
            }
            crate::logger::write_log(
                "Snapora:Session",
                &format!("会话 [{job_id}] 状态变更: {:?} -> {:?}", session.status, new_status),
            );
            session.status = new_status;
            Ok(())
        } else {
            Err(Error::StaleSession("无活跃会话".to_string()))
        }
    }

    /// 获取会话的初始化 Payload
    pub fn get_init_payload(&self, job_id: Option<&str>) -> Option<InitPayload> {
        let guard = self.inner.lock().unwrap();
        if let Some(ref session) = *guard {
            if let Some(jid) = job_id {
                if session.job_id != jid {
                    return None;
                }
            }
            return Some(session.init_payload.clone());
        }
        None
    }

    /// 完成截图会话并唤醒等待中的 Promise/Oneshot 通道
    pub fn complete_session(&self, job_id: &str, result: ScreenshotResult) -> Result<(), Error> {
        let mut guard = self.inner.lock().unwrap();
        if let Some(session) = guard.as_ref() {
            if session.job_id != job_id {
                crate::logger::write_log(
                    "Snapora:Session",
                    &format!("⚠️ complete_session 忽略过期会话回调 [{job_id}] (当前活跃: [{}])", session.job_id),
                );
                return Err(Error::StaleSession(format!(
                    "complete_session 拒绝: job_id [{job_id}] 已被新会话取代"
                )));
            }
        } else {
            return Err(Error::StaleSession("complete_session 调用时未找到活跃会话".to_string()));
        }

        // 取出当前会话并发送完成事件
        let mut session = guard.take().unwrap();
        let summary = match &result {
            ScreenshotResult::Completed { data, bounds, output, .. } => {
                format!("status: completed, bytes: {}, bounds: {bounds:?}, output: {output:?}", data.len())
            }
            ScreenshotResult::Cancelled => "status: cancelled".to_string(),
            ScreenshotResult::Failed { code, message } => {
                format!("status: failed, code: {code}, message: {message}")
            }
        };
        crate::logger::write_log("Snapora:Session", &format!("会话 [{}] 正常完成并归档 ({summary})", session.job_id));
        if let Some(sender) = session.result_sender.take() {
            let _ = sender.send(result);
        }
        Ok(())
    }

    /// 取消会话。若指定 job_id，校验匹配；若未指定则取消当前会话
    pub fn cancel_session(&self, job_id: Option<&str>) -> Result<(), Error> {
        let mut guard = self.inner.lock().unwrap();
        if let Some(session) = guard.as_ref() {
            if let Some(jid) = job_id {
                if session.job_id != jid {
                    crate::logger::write_log(
                        "Snapora:Session",
                        &format!("⚠️ cancel_session 忽略过期会话 [{jid}] (当前活跃: [{}])", session.job_id),
                    );
                    return Err(Error::StaleSession(format!(
                        "cancel_session 拒绝: job_id [{jid}] 已失效"
                    )));
                }
            }
        } else {
            return Ok(());
        }

        let mut session = guard.take().unwrap();
        crate::logger::write_log("Snapora:Session", &format!("会话 [{}] 显式取消", session.job_id));
        if let Some(sender) = session.result_sender.take() {
            let _ = sender.send(ScreenshotResult::Cancelled);
        }
        Ok(())
    }
}
