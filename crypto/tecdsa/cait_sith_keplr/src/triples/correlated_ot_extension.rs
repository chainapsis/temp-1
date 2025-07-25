/// Parameters we need for the correlated OT.
#[derive(Debug, Clone, Copy)]
pub struct CorrelatedOtParams<'sid> {
    pub(crate) sid: &'sid [u8],
    pub(crate) batch_size: usize,
}
