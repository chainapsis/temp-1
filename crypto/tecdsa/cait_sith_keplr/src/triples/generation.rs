use super::{TriplePub, TripleShare};

/// The output of running the triple generation protocol.
pub type TripleGenerationOutput<C> = (TripleShare<C>, TriplePub<C>);
