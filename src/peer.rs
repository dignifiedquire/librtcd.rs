
const SESSION_ID_SIZE: usize = 16;

pub struct PeerConnection {}

impl PeerConnection {
    pub fn new() -> PeerConnection {
        PeerConnection {}
    }

    /// Initialize a new peer connection
    pub fn initialize(&self) {}

    /// Stop the connection and cleanup
    pub fn close(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
