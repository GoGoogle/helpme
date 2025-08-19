// custom/role.rs
// 提供当前构建角色（Host 被控 / Controller 主控）的编译时标志

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Host,       // 被控端
    Controller, // 主控端
}

#[cfg(feature = "role_host")]
pub const CURRENT_ROLE: Role = Role::Host;

#[cfg(feature = "role_controller")]
pub const CURRENT_ROLE: Role = Role::Controller;

// 辅助函数：判断功能是否启用
pub fn is_host() -> bool {
    CURRENT_ROLE == Role::Host
}

pub fn is_controller() -> bool {
    CURRENT_ROLE == Role::Controller
}
