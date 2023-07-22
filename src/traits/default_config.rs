use serde::Serialize;

pub trait DefaultConfig {}

impl<T: Serialize> DefaultConfig for T {}
