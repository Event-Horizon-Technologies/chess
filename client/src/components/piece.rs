use async_std::channel::Receiver;
use dioxus::html::geometry::ClientPoint;
use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct PieceProps {
    image: String,
    top_left_starting: ClientPoint,
    size: u32,
    is_dragging: bool,
}

pub fn Piece(cx: Scope<PieceProps>) -> Element {
    let top_left = use_state(cx, || cx.props.top_left_starting);
    let drag_offset_receiver = cx.consume_context::<Receiver<ClientPoint>>().unwrap();
    let z_index = cx.props.is_dragging as u32 + 1; // 🏌️

    use_future(
        cx,
        (&cx.props.is_dragging, &cx.props.top_left_starting),
        |(is_dragging, top_left_starting)| {
            to_owned![top_left];
            async move {
                if is_dragging {
                    while let Ok(offset) = drag_offset_receiver.recv().await {
                        top_left.set(ClientPoint::new(
                            top_left_starting.x + offset.x,
                            top_left_starting.y + offset.y,
                        ));
                    }
                } else {
                    top_left.set(top_left_starting);
                }
            }
        },
    );

    cx.render(rsx! {
        img {
            src: "{cx.props.image}",
            class: "images",
            style: "left: {top_left.get().x}px; top: {top_left.get().y}px; z-index: {z_index}",
            width: "{cx.props.size}",
            height: "{cx.props.size}"
        }
    })
}
