use crate::BodyReader;
use futures::StreamExt;

impl tokio::io::AsyncRead for BodyReader {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let this = self.get_mut();
        match this.body.poll_next_unpin(cx) {
            std::task::Poll::Ready(Some(Ok(chunk))) => {
                buf.put_slice(&chunk);
                std::task::Poll::Ready(Ok(()))
            }
            std::task::Poll::Ready(Some(Err(e))) => std::task::Poll::Ready(Err(
                std::io::Error::new(std::io::ErrorKind::Other, format!("BodyReader: {}", e)),
            )),
            std::task::Poll::Ready(None) => std::task::Poll::Ready(Ok(())),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}
