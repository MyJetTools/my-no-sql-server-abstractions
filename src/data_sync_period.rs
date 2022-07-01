#[derive(Debug, Clone, Copy)]
pub enum DataSyncronizationPeriod {
    Immediately,
    Sec1,
    Sec5,
    Sec15,
    Sec30,
    Min1,
    Asap,
}
