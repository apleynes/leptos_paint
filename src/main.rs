use gloo_events::EventListener;
use leptos::{prelude::*, html::Canvas};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, CanvasRenderingContext2d, Element, HtmlCanvasElement};

fn draw_point(ctx: &CanvasRenderingContext2d, x: f64, y: f64, erase: bool, point_size: f64) {
    if erase {
        ctx.set_fill_style_str("white");
    } else {
        ctx.set_fill_style_str("black");
    }
    // Center the point
    ctx.fill_rect(x - point_size / 2.0, y - point_size / 2.0, point_size, point_size);
}

#[component]
fn PointSizeSlider(point_size: ReadSignal<f64>, set_point_size: WriteSignal<f64>) -> impl IntoView {
    view! {
        <input type="range" min="1" max="100" value=point_size on:input=move |evt| set_point_size.set(event_target_value(&evt).parse().unwrap()) />
    }
}

#[component]
fn App() -> impl IntoView {
    // signal: true = erase, false = draw
    let (is_erase, set_erase) = signal(false);
    let (point_size, set_point_size) = signal(4.0);
    // ref to the canvas element
    let canvas_ref = NodeRef::<Canvas>::new();

    // set up pointer listeners once the canvas is in the DOM
    Effect::new(move |_| {
        let canvas = canvas_ref
            .get()
            .expect("canvas should be in the DOM");
        // get 2D context
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        // track whether pointer is down
        let is_drawing = std::rc::Rc::new(std::cell::Cell::new(false));
        let drawing_flag = is_drawing.clone();
        let erase_flag = is_erase;

        // mousedown → start drawing
        // let canvas_clone = canvas.clone();
        EventListener::new(&canvas, "pointerdown", move |evt| {
            drawing_flag.set(true);
            let pe = evt.dyn_ref::<web_sys::PointerEvent>().unwrap();
            let canvas = canvas_ref.get().unwrap();
            let rect = canvas.get_bounding_client_rect();
            // adjust for canvas position
            let x = pe.client_x() as f64 - rect.left();
            let y = pe.client_y() as f64 - rect.top();
            // let x = pe.client_x() as f64;
            // let y = pe.client_y() as f64;
            draw_point(&ctx, x, y, erase_flag.get(), point_size.get());
        })
        .forget();

        // pointerup anywhere → stop drawing
        let drawing_flag_up = is_drawing.clone();
        EventListener::new(&window().unwrap(), "pointerup", move |_| {
            drawing_flag_up.set(false);
        })
        .forget();

        // pointermove → draw if pointerdown
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();  // Refresh context
        EventListener::new(&canvas, "pointermove", move |evt| {
            if !is_drawing.get() {
                return;
            }
            let pe = evt.dyn_ref::<web_sys::PointerEvent>().unwrap();
            let canvas = canvas_ref.get().unwrap();
            let rect = canvas.get_bounding_client_rect();
            let x = pe.client_x() as f64 - rect.left();
            let y = pe.client_y() as f64 - rect.top();
            // let x = pe.client_x() as f64;
            // let y = pe.client_y() as f64;
            draw_point(&ctx, x, y, erase_flag.get(), point_size.get());
        })
        .forget();
    });

    view! {
        <div>
            <button on:click=move |_| set_erase.set(false)>
                "Draw"
            </button>
            <button on:click=move |_| set_erase.set(true)>
                "Erase"
            </button>
            <canvas
                node_ref=canvas_ref
                width=500
                height=500
                style="border:1px solid black; background:white;"
            />
            <PointSizeSlider point_size=point_size set_point_size=set_point_size />
        </div>
    }
}


fn main() {
    // mount the app to <body>
    mount_to_body(|| view! { <App/> });
}
