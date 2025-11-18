use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::websocket::{ChannelsActor, WsSession};

#[get("/ws/{channel}")]
async fn get(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<String>,
    channels: web::Data<actix::Addr<ChannelsActor>>,
) -> actix_web::Result<HttpResponse> {
    let channel_name = path.into_inner();
    ws::start(
        WsSession {
            channel_name,
            channels: channels.get_ref().clone(),
        },
        &req,
        stream,
    )
}
