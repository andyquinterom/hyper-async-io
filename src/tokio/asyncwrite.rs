use crate::BodyWriter;

impl tokio::io::AsyncWrite for BodyWriter {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        let sender = &mut self.get_mut().sender;
        match sender.poll_ready(cx) {
            std::task::Poll::Ready(_) => {
                match sender.try_send_data(hyper::body::Bytes::copy_from_slice(buf)) {
                    Ok(_) => std::task::Poll::Ready(Ok(buf.len())),
                    Err(_) => std::task::Poll::Pending,
                }
            }
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
}
