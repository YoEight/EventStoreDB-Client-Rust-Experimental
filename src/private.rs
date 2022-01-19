use crate::{All, EventData, Single, Streaming};

pub trait Sealed {}

impl Sealed for usize {}
impl Sealed for EventData {}
impl<A: Sealed> Sealed for Vec<A> {}
impl<I> Sealed for Streaming<I> {}
impl Sealed for All {}
impl Sealed for Single {}
