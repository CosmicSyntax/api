use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use actix_web::{
    get,
    web::{self, Bytes},
    HttpRequest, HttpResponse,
};
use futures::{Future, Stream};
use tokio::time::{sleep, Instant, Sleep};
use tracing::instrument;

use crate::error::ApiErrors;

#[get("")]
#[instrument]
async fn stream(req: HttpRequest) -> Result<HttpResponse, ApiErrors> {
    let mut http = HttpResponse::Ok();
    http.content_type("text/html; charset=utf-8");
    Ok(http.streaming(Streamertastic(0, sleep(Duration::from_secs(1)))))
}

struct Streamertastic(u8, Sleep);

impl Stream for Streamertastic {
    type Item = Result<Bytes, ApiErrors>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let (mut sleep, count) = unsafe {
            let this = self.get_unchecked_mut();
            (Pin::new_unchecked(&mut this.1), &mut this.0)
        };

        match sleep.as_mut().poll(cx) {
            Poll::Ready(_) => {
                if *count == 100 {
                    return Poll::Ready(None);
                }

                *count += 1;

                sleep.reset(Instant::now() + Duration::from_millis(500));
                Poll::Ready(Some(Ok("hello world!".into())))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

pub fn config_stream(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/stream").service(stream));
}
