#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxLevel {
    Restricted,
    Standard,
    Trusted,
}
