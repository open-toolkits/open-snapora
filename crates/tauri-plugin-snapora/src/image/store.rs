use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use super::buffer::ImageBuffer;

/// 某任务关联的图像存储槽位
#[derive(Default, Clone)]
struct JobImageSlot {
    /// 原生采集的原始屏幕图像（通常为极速编码的高画质 JPEG 或无损图）
    source: Option<ImageBuffer>,
    /// 用户在 Overlay 标注完成后输出的最终图像（通常为 PNG）
    output: Option<ImageBuffer>,
}

/// 集中式图像缓存存储库（ImageStore）
/// 
/// 负责高分辨率位图在内存中的生命周期，与 Session 生命周期严格解耦。
/// Session 仅保存 ImageRef 标识，大型二进制在此集中管理。
#[derive(Clone, Default)]
pub struct ImageStore {
    inner: Arc<Mutex<HashMap<String, JobImageSlot>>>,
}

impl ImageStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 存储指定会话的原始全屏底图
    pub fn store_source(&self, job_id: &str, image: ImageBuffer) {
        let mut guard = self.inner.lock().unwrap();
        let slot = guard.entry(job_id.to_string()).or_default();
        slot.source = Some(image);
    }

    /// 获取指定会话的原始全屏底图
    pub fn get_source(&self, job_id: &str) -> Option<ImageBuffer> {
        let guard = self.inner.lock().unwrap();
        guard.get(job_id).and_then(|slot| slot.source.clone())
    }

    /// 存储指定会话在编辑/导出阶段生成的输出图像
    pub fn store_output(&self, job_id: &str, image: ImageBuffer) {
        let mut guard = self.inner.lock().unwrap();
        let slot = guard.entry(job_id.to_string()).or_default();
        slot.output = Some(image);
    }

    /// 获取指定会话的输出图像
    pub fn get_output(&self, job_id: &str) -> Option<ImageBuffer> {
        let guard = self.inner.lock().unwrap();
        guard.get(job_id).and_then(|slot| slot.output.clone())
    }

    /// 获取当前最新的输出图像字节（若 job_id 为空或未找到特定 session，尝试获取任一最新 output 兜底）
    pub fn get_latest_output(&self) -> Option<ImageBuffer> {
        let guard = self.inner.lock().unwrap();
        for slot in guard.values() {
            if let Some(ref img) = slot.output {
                return Some(img.clone());
            }
        }
        None
    }

    /// 会话完成或取消后清理图片，释放内存
    pub fn remove(&self, job_id: &str) {
        let mut guard = self.inner.lock().unwrap();
        guard.remove(job_id);
    }

    /// 清空所有图片存储
    pub fn clear(&self) {
        let mut guard = self.inner.lock().unwrap();
        guard.clear();
    }
}
