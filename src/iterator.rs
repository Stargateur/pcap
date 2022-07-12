use crate::Activated;
use crate::Capture;
use crate::Error;
use crate::LendingIterator;
use crate::Packet;

/// Implement an Iterator of Packet
pub struct PacketIter<S: Activated + ?Sized> {
    capture: Capture<S>,
}

impl<S: Activated + ?Sized> PacketIter<S> {
    pub(crate) fn new(capture: Capture<S>) -> Self {
        Self { capture }
    }

    pub fn capture_mut(&mut self) -> &mut Capture<S> {
        &mut self.capture
    }
}

impl<S: Activated + ?Sized> From<PacketIter<S>> for Capture<S> {
    fn from(iter: PacketIter<S>) -> Self {
        iter.capture
    }
}

impl<S: Activated + ?Sized> LendingIterator for PacketIter<S> {
    type Item<'a> = Result<Packet<'a>, Error> where S: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        match self.capture.next() {
            Ok(packet) => Some(Ok(packet)),
            Err(Error::NoMorePackets) => None,
            Err(e) => Some(Err(e)),
        }
    }
}
