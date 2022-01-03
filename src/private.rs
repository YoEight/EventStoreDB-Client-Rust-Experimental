use crate::{All, EventData, Single};

pub trait Sealed {}

impl Sealed for usize {}
impl Sealed for EventData {}
impl<A: Sealed> Sealed for Vec<A> {}
impl Sealed for All {}
impl Sealed for Single {}
