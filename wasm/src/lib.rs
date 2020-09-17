use crate::widgets::{
    rect::{Rect, RectState},
    smiley::Smiley,
    {Draw, RectGeometry, Settings, UiGlobalState, WidgetState},
};
use enclose::enc;
use nalgebra::Point2;
use piet::{Color, RenderContext};
use piet_web::WebRenderContext;
use seed::{attrs, canvas, log, prelude::*, App};
use serde::{Deserialize, Serialize};
use std::env;
use web_sys::HtmlCanvasElement;

mod widgets;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
pub fn start() -> Box<[JsValue]> {
    let app = App::start("wasm", init, update, view);

    create_closures_for_js(&app)
}

fn create_closures_for_js(app: &App<Msg, Model, Node<Msg>>) -> Box<[JsValue]> {
    let activate_events;
    {
        let closure = Closure::wrap(Box::new(enc!((app) move |active: bool| {
            if active {
                app.update(Msg::Subscribe)
            } else {
                app.update(Msg::Unsubscribe)
            }
        })) as Box<dyn FnMut(bool)>);
        activate_events = closure.as_ref().clone();
        closure.forget();
    }

    let add_widget;
    {
        let closure = Closure::wrap(Box::new(enc!((app) move || {
            app.update(Msg::AddWidget)
        })) as Box<dyn FnMut()>);
        add_widget = closure.as_ref().clone();
        closure.forget();
    }

    let update_widget;
    {
        // state: WidgetState
        let closure = Closure::wrap(Box::new(enc!((app) move |state: JsValue| {
            app.update(Msg::UpdateWidget(state))
        })) as Box<dyn FnMut(JsValue)>);
        update_widget = closure.as_ref().clone();
        closure.forget();
    }

    let select_widget;
    {
        let closure = Closure::wrap(Box::new(enc!((app) move |uuid: Option<String>| {
            app.update(Msg::SelectWidget(uuid))
        })) as Box<dyn FnMut(Option<String>)>);
        select_widget = closure.as_ref().clone();
        closure.forget();
    }

    let hover_widget;
    {
        let closure = Closure::wrap(Box::new(enc!((app) move |uuid: Option<String>| {
            app.update(Msg::HoverWidget(uuid))
        })) as Box<dyn FnMut(Option<String>)>);
        hover_widget = closure.as_ref().clone();
        closure.forget();
    }

    let toggle_visibility_widget;
    {
        let closure = Closure::wrap(Box::new(enc!((app) move |uuid: Option<String>| {
            app.update(Msg::ToggleVisibilityWidget(uuid))
        })) as Box<dyn FnMut(Option<String>)>);
        toggle_visibility_widget = closure.as_ref().clone();
        closure.forget();
    }

    let delete_widget;
    {
        let closure = Closure::wrap(Box::new(enc!((app) move |uuid: Option<String>| {
            app.update(Msg::DeleteWidget(uuid))
        })) as Box<dyn FnMut(Option<String>)>);
        delete_widget = closure.as_ref().clone();
        closure.forget();
    }

    let swap_widget;
    {
        let closure = Closure::wrap(Box::new(enc!((app) move |state: JsValue| {
            app.update(Msg::SwapWidget(state))
        })) as Box<dyn FnMut(JsValue)>);
        swap_widget = closure.as_ref().clone();
        closure.forget();
    }

    let update_settings;
    {
        let closure = Closure::wrap(Box::new(enc!((app) move |settings: JsValue| {
            app.update(Msg::UpdateSettings(settings))
        })) as Box<dyn FnMut(JsValue)>);
        update_settings = closure.as_ref().clone();
        closure.forget();
    }

    vec![
        activate_events,
        add_widget,
        update_widget,
        select_widget,
        hover_widget,
        toggle_visibility_widget,
        delete_widget,
        swap_widget,
        update_settings,
    ]
    .into_boxed_slice()
}

#[wasm_bindgen]
extern "C" {
    fn update_app_state(app_state: &JsValue, widgets: &JsValue);
}

// --- Colors ---
const WHITE: Color = Color::rgb8(0xFF, 0xFF, 0xFF);
const BLUE: Color = Color::rgb8(0x00, 0x88, 0xCC);
const PINK: Color = Color::rgb8(0xB8, 0x2E, 0xE5);

// --- Application ---
pub enum WidgetType {
    Smiley,
}

// `init` describes what should happen when your app started.
pub fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(|_| Msg::Rendered);
    let mut model = Model::default();

    // WebSocket TODO: Make it better
    let web_server = env::var("SERVER_URL").or_else("0.0.0.0:8080");
    let web_socket = WebSocket::builder(format!("ws://{}/ws/", web_server), orders)
        .on_open(Msg::SendMessage)
        .on_message(Msg::MessageReceived)
        .build_and_open();

    // TODO: (later) Serialize (serde) for loading/saving
    model.widgets = vec![
        Smiley::new("Smiley Pro", (275.0, 300.0), 150.0),
        Smiley::new("Smiley", (475.0, 110.0), 50.0),
        Smiley::new("Smiley Mini", (550.0, 165.0), 30.0),
    ];

    if let Ok(ws) = web_socket {
        model.web_socket = Some(ws);
    }

    model
}

// `Model` describes our app state.
#[derive(Default)]
pub struct Model {
    event_streams: Vec<StreamHandle>, // Window Events
    key_code: String,                 // Keyboard key pressed
    canvas: ElRef<HtmlCanvasElement>, // Fluss Canvas. Keep the context instead?
    widgets: Vec<Smiley>,
    app_state: UiGlobalState, // UI global state
    web_socket: Option<WebSocket>,
}

// `Msg` describes the different events you can modify state with.
pub enum Msg {
    Subscribe,
    Unsubscribe,
    Rendered, // DOM Component rendered
    MouseMoved(web_sys::MouseEvent),
    MouseDown(web_sys::MouseEvent),
    MouseUp,
    MouseOut,
    KeyPressed(web_sys::KeyboardEvent),
    WindowResize,
    AddWidget,
    UpdateWidget(JsValue), // Widget
    SelectWidget(Option<String>),
    HoverWidget(Option<String>),
    ToggleVisibilityWidget(Option<String>),
    DeleteWidget(Option<String>),
    SwapWidget(JsValue),
    UpdateSettings(JsValue), // Settings
    Draw,
    MessageReceived(WebSocketMessage),
    SendMessage(),
}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Rendered => {
            // draw(&model.canvas, model.fill_color);
            // We want to call `.skip` to prevent infinite loop.
            // (However infinite loops are useful for animations.)
            // orders.after_next_render(|_| Msg::Rendered).skip();
            // TODO Check https://github.com/seed-rs/seed/blob/2b134d1de2a8b9aa520d11be6e45eef1e5fcd527/examples/subscribe/src/lib.rs#L15-L18
            orders.stream(streams::window_event(Ev::Resize, |_| Msg::WindowResize));
            orders.send_msg(Msg::Subscribe);
            orders.send_msg(Msg::WindowResize);
        }
        Msg::Subscribe => {
            if model.event_streams.is_empty() {
                model.event_streams = vec![orders
                    .stream_with_handle(streams::window_event(Ev::KeyDown, |event| {
                        Msg::KeyPressed(event.unchecked_into())
                    }))];
            }
        }
        Msg::Unsubscribe => {
            model.event_streams.clear();
        }
        Msg::MouseMoved(event) => {
            model.app_state.cursor.previous_position = &*model.app_state.cursor.position;
            model.app_state.cursor.position = Some(Point2::new(
                event.client_x().into(),
                event.client_y().into(),
            ));
            orders.send_msg(Msg::Draw);
        }
        Msg::MouseDown(event) => {
            // Needed to avoid the canvas to take the focus control
            // Focus is given to the Layer DOM Element from "app"
            event.prevent_default();
            model.app_state.cursor.down_start_position = Some(Point2::new(
                event.client_x().into(),
                event.client_y().into(),
            ));

            // If anchor is hovered, we keep the current selection uuid
            if !model.app_state.cursor.is_active {
                model.app_state.active_widget_uuid = None;
            }

            orders.send_msg(Msg::Draw);
        }
        Msg::MouseUp => {
            model.app_state.cursor.down_start_position = None;
            orders.send_msg(Msg::Draw);
        }
        Msg::MouseOut => {
            model.app_state.cursor.position = None;
            orders.send_msg(Msg::Draw);
        }
        Msg::KeyPressed(event) => {
            model.key_code = event.code();

            // prevent some key on the entire application
            match model.key_code.as_str() {
                // browser back
                "Backspace" => {
                    event.prevent_default();
                }
                _ => {}
            }

            if let Some(select_widget_uuid) = &model.app_state.active_widget_uuid {
                let selected_widget = model
                    .widgets
                    .iter_mut()
                    .find(|w| w.uuid.eq(select_widget_uuid));

                if let Some(widget) = selected_widget {
                    let offset = if event.shift_key() { 10. } else { 1. };

                    match model.key_code.as_str() {
                        "ArrowUp" => {
                            widget.geometry.y -= offset;
                            orders.send_msg(Msg::Draw);
                        }
                        "ArrowDown" => {
                            widget.geometry.y += offset;
                            orders.send_msg(Msg::Draw);
                        }
                        "ArrowLeft" => {
                            widget.geometry.x -= offset;
                            orders.send_msg(Msg::Draw);
                        }
                        "ArrowRight" => {
                            widget.geometry.x += offset;
                            orders.send_msg(Msg::Draw);
                        }
                        "Escape" => {
                            model.app_state.active_widget_uuid = None;
                            orders.send_msg(Msg::Draw);
                        }
                        "Delete" => {
                            orders.send_msg(Msg::DeleteWidget(Some(widget.uuid.clone())));
                        }
                        "Backspace" => {
                            orders.send_msg(Msg::DeleteWidget(Some(widget.uuid.clone())));
                        }
                        "KeyH" => {
                            orders.send_msg(Msg::ToggleVisibilityWidget(Some(widget.uuid.clone())));
                        }
                        "KeyL" => {
                            model.app_state.settings.keep_ratio =
                                !model.app_state.settings.keep_ratio;
                            orders.send_msg(Msg::Draw);
                        }
                        _ => {}
                    }
                }
            }
        }
        Msg::WindowResize => {
            // Set canvas size
            if let Some(canvas) = model.canvas.get() {
                let window = seed::browser::util::window(); // TODO Use event param?

                if let Ok(width) = window.inner_width() {
                    let width = width.as_f64().unwrap();
                    model.app_state.canvas_geometry = RectGeometry {
                        width,
                        ..model.app_state.canvas_geometry
                    };
                    canvas.set_width(width as u32);
                }

                if let Ok(height) = window.inner_height() {
                    let height = height.as_f64().unwrap();
                    model.app_state.canvas_geometry = RectGeometry {
                        height,
                        ..model.app_state.canvas_geometry
                    };
                    canvas.set_height(height as u32);
                }

                orders.send_msg(Msg::Draw);
            }
        }
        Msg::AddWidget => {
            model.widgets.insert(
                0,
                Smiley::new(
                    format!("New Smiley {}", model.widgets.len() + 1).as_ref(),
                    (575.0, 300.0),
                    63.0,
                ),
            );
            orders.send_msg(Msg::Draw);
        }
        Msg::SelectWidget(uuid) => {
            model.app_state.active_widget_uuid = uuid;
            orders.send_msg(Msg::Draw);
        }
        Msg::HoverWidget(uuid) => {
            model.app_state.pointer_widget_uuid = uuid;
            orders.send_msg(Msg::Draw);
        }
        Msg::UpdateWidget(state) => {
            let serialized_state: serde_json::Result<Smiley> = state.as_ref().into_serde();
            if let Ok(mut state) = serialized_state {
                let selected_widget = model.widgets.iter_mut().find(|w| w.uuid.eq(&state.uuid));
                if let Some(widget) = selected_widget {
                    if model.app_state.settings.keep_ratio {
                        let old_geometry = widget.geometry;
                        let error = 0.01f64; // Use an epsilon for comparison
                        if (old_geometry.width - state.geometry.width).abs() > error {
                            // width is different, change height
                            let diff = state.geometry.width / old_geometry.width;
                            state.geometry.height = old_geometry.height * diff;
                        } else if (old_geometry.height - state.geometry.height).abs() > error {
                            // height is diffenrent, change width
                            let diff = state.geometry.height / old_geometry.height;
                            state.geometry.width = old_geometry.width * diff;
                        }
                    }

                    widget.name = state.name;
                    widget.geometry = state.geometry;
                    // TODO initial_geometry?
                    // TODO check validity/parse
                    // "1.23".parse::<f64>().is_ok();
                }
                orders.send_msg(Msg::Draw);
            }
        }
        Msg::ToggleVisibilityWidget(uuid) => {
            if let Some(uuid) = uuid {
                let selected_widget = model.widgets.iter_mut().find(|w| w.uuid.eq(&uuid));
                if let Some(widget) = selected_widget {
                    widget.visible = !widget.visible;
                    orders.send_msg(Msg::Draw);
                }
            }
        }
        Msg::DeleteWidget(uuid) => {
            if let Some(uuid) = uuid {
                model.widgets.retain(|w| w.uuid.ne(&uuid));
                orders.send_msg(Msg::Draw);
            }
        }
        Msg::SwapWidget(state) => {
            #[derive(Serialize, Deserialize)]
            struct Test {
                uuid: String,
                a: usize,
                b: usize,
            }

            let serialized_state: serde_json::Result<Test> = state.as_ref().into_serde();
            if let Ok(state) = serialized_state {
                // TODO check binary_search_by_key(&13, |&(a,b)| b)
                //let selected_widget = model.widgets.iter_mut().find(|w| w.uuid.eq(&state.uuid));
                //if selected_widget.is_some() {
                let len = model.widgets.len();
                if state.a < len && state.b < len {
                    model.widgets.swap(state.a, state.b);
                    orders.send_msg(Msg::Draw);
                }
                //}
            }
        }
        Msg::UpdateSettings(settings) => {
            let serialized_settings: serde_json::Result<Settings> = settings.as_ref().into_serde();
            if let Ok(settings) = serialized_settings {
                model.app_state.settings = settings;
                orders.send_msg(Msg::Draw);
            }
        }
        Msg::Draw => {
            draw(&model.canvas, &mut model.app_state, &mut model.widgets);
        }
        Msg::MessageReceived(message) => {
            log! {"received"}
            log! {message}
        }
        Msg::SendMessage() => {
            if let Some(ws) = &model.web_socket {
                ws.send_text("Hello, websocket").unwrap();
            }
        }
    }
}

fn draw(
    canvas: &ElRef<HtmlCanvasElement>,
    mut app_state: &mut UiGlobalState,
    widgets: &mut [Smiley],
) {
    // TODO It is probably very un-optimized to do that!
    let window = seed::browser::util::window();
    let canvas = canvas.get().expect("get canvas element");
    let context = seed::canvas_context_2d(&canvas);

    let dpr = window.device_pixel_ratio();
    canvas.set_width((app_state.canvas_geometry.width * dpr) as u32);
    canvas.set_height((app_state.canvas_geometry.height * dpr) as u32);
    let _ = context.scale(dpr, dpr);

    let mut ctx = WebRenderContext::new(context, window);
    let cursor = app_state.cursor;

    ctx.clear(Color::rgb8(0xf9, 0xf9, 0xf9));

    // --- Update ---

    // -> Updated Widgets
    let mut widgets_states: Vec<WidgetState> = vec![];

    // Need to be able to hover or select it from the layer
    // for widget in widgets.iter_mut().filter(|w| w.visible).rev() {
    for widget in widgets.iter_mut().rev() {
        widgets_states.push(widget.update(&mut app_state));
    }

    // -> Update Rects around the widgets
    let mut selected_widget_state: Option<&WidgetState> = None;
    let mut hovered_widget_state: Option<&WidgetState> = None;

    for state in widgets_states.iter() {
        let mut do_not_hover = false;

        // Selected Widget from canvas/pointer
        if state.selected {
            app_state.active_widget_uuid = Some(String::from(&state.uuid));
        }

        // Maybe selected Widget from layers (or from canvas)
        if let Some(uuid) = &app_state.active_widget_uuid {
            if uuid.eq(&state.uuid) {
                selected_widget_state = Some(state);
                do_not_hover = true;
            }
        }

        // Hovered Widget from canvas/pointer
        if state.hovered {
            app_state.pointer_widget_uuid = Some(String::from(&state.uuid));
        }

        // Maybe hovered Widget from layers (or from canvas)
        if let Some(uuid) = &app_state.pointer_widget_uuid {
            if !do_not_hover && uuid.eq(&state.uuid) {
                hovered_widget_state = Some(state);
            }
        }

        // INFO: Keep here, the above test happens sometimes twice and fails the hover Rect
        if state.hovered {
            hovered_widget_state = if do_not_hover { None } else { Some(state) };
        }
    }

    // -> Update Selected Widget from Rect (anchors)
    let mut selection_rect: Option<Rect> = None;

    if let Some(state) = selected_widget_state {
        let mut rect = Rect::new(state.geometry, RectState::Select);
        // Difference between previous and current frame for the selected Rect (from pointer)
        // FIXME maybe return None (see Draw trait)
        let geometry_difference = rect.update(&mut app_state);

        if let Some(select_widget_uuid) = &app_state.active_widget_uuid {
            let selected_widget = widgets.iter_mut().find(|w| w.uuid.eq(select_widget_uuid));

            if let Some(widget) = selected_widget {
                // Update Widget from Rect (anchors)
                widget.change(&geometry_difference);
                // Reset geometry for selected Rect
                rect.reset_geometry(&widget.geometry);
            }
        }

        selection_rect = Some(rect);
    }

    // --- Draw ---

    // -> Draw all visible widgets
    for widget in widgets.iter_mut().filter(|w| w.visible).rev() {
        widget.draw(&mut ctx, &app_state);
    }

    // -> Draw the selection Rect
    if let Some(rect) = selection_rect {
        rect.draw(&mut ctx, &app_state);
    }

    // -> Draw the hover Rect
    if let Some(state) = hovered_widget_state {
        // Avoid hovering Widget while doing modification
        if cursor.down_start_position.is_none() {
            let rect = Rect::new(state.geometry, RectState::Hover);
            rect.draw(&mut ctx, &app_state);
        }
    }

    // -> Draw the cursor
    cursor.draw(&mut ctx, &app_state);

    // --- Update Web UI / JavaScript ---
    // TODO Should send only on change, currently send all the time
    update_app_state(
        &JsValue::from_serde(&app_state).unwrap(),
        &JsValue::from_serde(&widgets).unwrap(),
    );

    // --- Finish & Reset ---
    ctx.finish().unwrap();
    app_state.pointer_widget_uuid = None;
}

// `view` describes what to display.
pub fn view(model: &Model) -> Node<Msg> {
    canvas![
        el_ref(&model.canvas),
        attrs![
            At::Width => px(200),
            At::Height => px(100),
        ],
        ev(Ev::MouseMove, |event| Msg::MouseMoved(
            event.unchecked_into()
        )),
        ev(Ev::MouseDown, |event| Msg::MouseDown(
            event.unchecked_into()
        )),
        ev(Ev::MouseUp, |_| Msg::MouseUp),
        ev(Ev::MouseOut, |_| Msg::MouseOut),
    ]
}
