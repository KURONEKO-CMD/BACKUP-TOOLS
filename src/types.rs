// src/types.rs
#[derive(Copy, Clone, Debug)]
pub enum Mode {
    /// 增量：目标存在则跳过
    Sync,
    /// 全覆盖：总是复制（临时文件 + rename）
    Replace,
}
