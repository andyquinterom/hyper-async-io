pub mod tokio;

pub struct BodyWriter {
    sender: hyper::body::Sender,
}

impl BodyWriter {
    pub fn channel() -> (Self, hyper::Body) {
        let (sender, body) = hyper::body::Body::channel();
        (Self { sender }, body)
    }
}

pub struct BodyReader {
    body: hyper::body::Body,
}

impl BodyReader {
    pub fn new(body: hyper::Body) -> Self {
        Self { body }
    }
}

impl From<hyper::Body> for BodyReader {
    fn from(body: hyper::Body) -> Self {
        Self::new(body)
    }
}
