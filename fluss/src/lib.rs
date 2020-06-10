use crate::widgets::GeometryChangeState;
use nalgebra::Point2;
use piet::{Color, RenderContext};
use piet_web::WebRenderContext;
use seed::{a, attrs, canvas, div, h5, i, input, label, li, prelude::*, span, ul, App, C};
use web_sys::HtmlCanvasElement;
use widgets::{
    rect::{Rect, RectState},
    smiley::{Geometry, Smiley},
    {Draw, RectGeometry, UiGlobalState, WidgetState},
};

mod widgets;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    App::start("wasm", init, update, view);

    Ok(())
}

#[wasm_bindgen(js_name = Foo)]
pub struct JsFoo {
    name: String,
}

#[wasm_bindgen(js_class = Foo)]
impl JsFoo {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> Self {
        JsFoo {
            name: String::from("world"),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

// --- Settings ---
pub static POINTER_SIZE: f64 = 11.0;
pub static RECT_BORDER_WIDTH: f64 = 2.0;
pub static RECT_ANCHOR_RADIUS: f64 = 5.0;

// --- Application ---
pub enum WidgetType {
    Smiley,
}

// `init` describes what should happen when your app started.
pub fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(|_| Msg::Rendered);
    let mut model = Model::default();

    // TODO (later) Serialize (serde) for loading/saving
    model.widgets = vec![
        Smiley::new("Smiley Pro", (275.0, 300.0), 150.0),
        Smiley::new("Smiley", (75.0, 110.0), 50.0),
        Smiley::new("Smiley Mini", (550.0, 165.0), 30.0),
    ];

    model
}

// `Model` describes our app state.
#[derive(Default)]
pub struct Model {
    event_streams: Vec<StreamHandle>, // Window Events
    key_code: String,                 // Keyboard key pressed
    canvas: ElRef<HtmlCanvasElement>, // Fluss Canvas TODO Keep the context instead?
    widgets: Vec<Smiley>,             // List of Widgets on Canvas
    ui_state: UiGlobalState,          // UI global state
}

// `Msg` describes the different events you can modify state with.
pub enum Msg {
    Rendered, // DOM Component rendered
    MouseMoved(web_sys::MouseEvent),
    MouseDown(web_sys::MouseEvent),
    MouseUp,
    MouseOut,
    KeyPressed(web_sys::KeyboardEvent),
    WindowResize,
    AddWidget,
    UpdateWidget(String, Geometry),
    Draw,
}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Rendered => {
            // draw(&model.canvas, model.fill_color);
            // We want to call `.skip` to prevent infinite loop.
            // (However infinite loops are useful for animations.)
            // orders.after_next_render(|_| Msg::Rendered).skip();
            // TODO Being able to deactivate KeyPressed. Useful when on input.
            if model.event_streams.is_empty() {
                model.event_streams = vec![
                    orders.stream_with_handle(streams::window_event(Ev::KeyDown, |event| {
                        Msg::KeyPressed(event.unchecked_into())
                    })),
                    orders.stream_with_handle(streams::window_event(Ev::Resize, |_| {
                        Msg::WindowResize
                    })),
                ];
            }

            orders.send_msg(Msg::WindowResize);
        }
        Msg::MouseMoved(event) => {
            model.ui_state.cursor.previous_position = model.ui_state.cursor.position.clone();
            model.ui_state.cursor.position = Some(Point2::new(
                event.client_x().into(),
                event.client_y().into(),
            ));
            orders.send_msg(Msg::Draw);
        }
        Msg::MouseDown(event) => {
            model.ui_state.cursor.down_start_position = Some(Point2::new(
                event.client_x().into(),
                event.client_y().into(),
            ));

            // If anchor is hovered, we keep the current selection uuid
            if !model.ui_state.cursor.is_active {
                model.ui_state.select_widget_uuid = None;
            }

            orders.send_msg(Msg::Draw);
        }
        Msg::MouseUp => {
            model.ui_state.cursor.down_start_position = None;
            orders.send_msg(Msg::Draw);
        }
        Msg::MouseOut => {
            model.ui_state.cursor.position = None;
            orders.send_msg(Msg::Draw);
        }
        Msg::KeyPressed(ev) => {
            model.key_code = ev.code();

            if let Some(select_widget_uuid) = &model.ui_state.select_widget_uuid {
                let selected_widget = model
                    .widgets
                    .iter_mut()
                    .find(|w| w.uuid.eq(select_widget_uuid));

                if let Some(widget) = selected_widget {
                    let offset = if ev.shift_key() { 10. } else { 1. };

                    match model.key_code.as_str() {
                        "ArrowUp" => {
                            widget.geometry.center = Point2::new(
                                widget.geometry.center.x,
                                widget.geometry.center.y - offset,
                            );
                            orders.send_msg(Msg::Draw);
                        }
                        "ArrowDown" => {
                            widget.geometry.center = Point2::new(
                                widget.geometry.center.x,
                                widget.geometry.center.y + offset,
                            );
                            orders.send_msg(Msg::Draw);
                        }
                        "ArrowLeft" => {
                            widget.geometry.center = Point2::new(
                                widget.geometry.center.x - offset,
                                widget.geometry.center.y,
                            );
                            orders.send_msg(Msg::Draw);
                        }
                        "ArrowRight" => {
                            widget.geometry.center = Point2::new(
                                widget.geometry.center.x + offset,
                                widget.geometry.center.y,
                            );
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
                    model.ui_state.canvas_geometry = RectGeometry {
                        width,
                        ..model.ui_state.canvas_geometry
                    };
                    canvas.set_width(width as u32);
                }

                if let Ok(height) = window.inner_height() {
                    let height = height.as_f64().unwrap();
                    model.ui_state.canvas_geometry = RectGeometry {
                        height,
                        ..model.ui_state.canvas_geometry
                    };
                    canvas.set_height(height as u32);
                }

                orders.send_msg(Msg::Draw);
            }
        }
        Msg::AddWidget => {
            model.widgets.push(Smiley::new(
                format!("New Smiley {}", model.widgets.len() + 1).as_ref(),
                (575.0, 300.0),
                63.0,
            ));
            orders.send_msg(Msg::Draw);
        }
        Msg::UpdateWidget(uuid, geometry) => {
            let selected_widget = model.widgets.iter_mut().find(|w| w.uuid.eq(&uuid));
            if let Some(widget) = selected_widget {
                widget.geometry = geometry;
            }
            orders.send_msg(Msg::Draw);
        }
        Msg::Draw => {
            draw(&model.canvas, &mut model.ui_state, &mut model.widgets);
        }
    }
}

fn draw(
    canvas: &ElRef<HtmlCanvasElement>,
    mut ui_state: &mut UiGlobalState,
    widgets: &mut [Smiley],
) {
    // TODO It looks un-optimized to do that?
    let window = seed::browser::util::window();
    let canvas = canvas.get().expect("get canvas element");
    let context = seed::canvas_context_2d(&canvas);
    let mut ctx = WebRenderContext::new(context, window);
    let cursor = ui_state.cursor;

    ctx.clear(Color::WHITE);

    // --- Update ---

    // Updated Widgets
    let mut widgets_states: Vec<WidgetState> = vec![];

    for widget in widgets.iter_mut() {
        widgets_states.push(widget.update(&mut ui_state));
    }

    // Update Rects around the widgets
    let mut selected_geometry: Option<&WidgetState> = None;
    let mut hovered_geometry: Option<&WidgetState> = None;

    for state in widgets_states.iter() {
        let mut do_not_hover = false;

        if let Some(uuid) = &ui_state.select_widget_uuid {
            if uuid.eq(&state.uuid) {
                selected_geometry = Some(state);
                do_not_hover = true;
            }
        }

        if state.selected {
            selected_geometry = Some(state);
            ui_state.select_widget_uuid = Some(String::from(&state.uuid));
            do_not_hover = true;
        }

        if state.hovered {
            hovered_geometry = if do_not_hover { None } else { Some(state) };
            ui_state.hover_widget_uuid = Some(String::from(&state.uuid));
        }
    }

    // Update selection Rect
    let mut selection_rect: Option<Rect> = None;

    if let Some(state) = selected_geometry {
        let mut rect = Rect::new(state.geometry, RectState::Select);
        let change_state = rect.update(&mut ui_state);

        if let Some(select_widget_uuid) = &ui_state.select_widget_uuid {
            let selected_widget = widgets.iter_mut().find(|w| w.uuid.eq(select_widget_uuid));

            if let Some(widget) = selected_widget {
                // Update Widget from anchor
                let new_state = widget.change(change_state);

                rect.change(GeometryChangeState {
                    geometry: new_state.geometry,
                });

                // TODO Update Rect from anchor
                // rect.change(RectGeometry {
                //     x: rect.geometry.x + change_state.geometry.x,
                //     y: rect.geometry.y + change_state.geometry.y,
                //     ..rect.geometry
                // });
            }
        }

        selection_rect = Some(rect);
    }

    // --- Draw ---

    // Draw all widgets
    for widget in widgets.iter_mut() {
        widget.draw(&mut ctx, &ui_state);
    }

    // Draw the selection Rect
    if let Some(rect) = selection_rect {
        rect.draw(&mut ctx, &ui_state);
    }

    // Draw the hover Rect (empty update)
    if let Some(state) = hovered_geometry {
        if cursor.down_start_position.is_none() {
            let rect = Rect::new(state.geometry, RectState::Hover);
            rect.draw(&mut ctx, &ui_state);
        }
    }

    // Draw the cursor
    cursor.draw(&mut ctx, &ui_state);
}

// `view` describes what to display.
pub fn view(model: &Model) -> impl IntoNodes<Msg> {
    let widgets = &model.widgets;
    let mut selected_widget: Option<&Smiley> = None;
    if let Some(select_widget_uuid) = &model.ui_state.select_widget_uuid {
        selected_widget = widgets.iter().find(|w| w.uuid.eq(select_widget_uuid));
    }

    vec![
        view_header(),
        view_widget_panel(selected_widget),
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
        ],
    ]
}

fn view_widget_panel(widget: Option<&Smiley>) -> Node<Msg> {
    let mut panel: Vec<Node<Msg>> = vec![];

    if let Some(current_widget) = widget {
        panel.push(div![
            C!["container"],
            h5![&current_widget.name],
            div![
                C!["form-group"],
                label!["Geometry"],
                div![
                    C!["input-group mb-3"],
                    div![
                        C!["input-group-prepend"],
                        span![C!["input-group-text"], "CX"],
                    ],
                    input![
                        C!["form-control"],
                        attrs![
                            At::Type => "number",
                            At::Value => &current_widget.geometry.center.x,
                            At::Pattern => "[0-9]*",
                        ],
                        {
                            let uuid = String::from(&current_widget.uuid);
                            let geometry = current_widget.geometry.clone();
                            input_ev(Ev::Input, move |value| {
                                Msg::UpdateWidget(
                                    uuid,
                                    Geometry {
                                        center: Point2::new(
                                            value.parse().unwrap(),
                                            geometry.center.y,
                                        ),
                                        radius: geometry.radius,
                                    },
                                )
                            })
                        },
                    ],
                    div![
                        C!["input-group-append"],
                        span![C!["input-group-text"], "px"],
                    ],
                ],
                div![
                    C!["input-group mb-3"],
                    div![
                        C!["input-group-prepend"],
                        span![C!["input-group-text"], "CY"],
                    ],
                    input![
                        C!["form-control"],
                        attrs![
                            At::Type => "number",
                            At::Value => &current_widget.geometry.center.y,
                            At::Pattern => "[0-9]*",
                        ],
                        {
                            let uuid = String::from(&current_widget.uuid);
                            let geometry = current_widget.geometry.clone();
                            input_ev(Ev::Input, move |value| {
                                Msg::UpdateWidget(
                                    uuid,
                                    Geometry {
                                        center: Point2::new(
                                            geometry.center.x,
                                            value.parse().unwrap(),
                                        ),
                                        radius: geometry.radius,
                                    },
                                )
                            })
                        },
                    ],
                    div![
                        C!["input-group-append"],
                        span![C!["input-group-text"], "px"],
                    ],
                ],
                div![
                    C!["input-group mb-3"],
                    div![
                        C!["input-group-prepend"],
                        span![C!["input-group-text"], "r"],
                    ],
                    input![
                        C!["form-control"],
                        attrs![
                            At::Type => "number",
                            At::Value => &current_widget.geometry.radius,
                            At::Pattern => "[0-9]*",
                        ],
                        {
                            let uuid = String::from(&current_widget.uuid);
                            let geometry = current_widget.geometry.clone();
                            input_ev(Ev::Input, move |value| {
                                Msg::UpdateWidget(
                                    uuid,
                                    Geometry {
                                        center: Point2::new(geometry.center.x, geometry.center.y),
                                        radius: value.parse().unwrap(),
                                    },
                                )
                            })
                        },
                    ],
                    div![
                        C!["input-group-append"],
                        span![C!["input-group-text"], "px"],
                    ],
                ],
            ],
        ]);
    }

    div![
        C!["sidebar bg-light border-left"],
        div![C!["sidebar-sticky"], panel]
    ]
}

fn view_header() -> Node<Msg> {
    div![
        C!["navbar navbar-expand navbar-dark fixed-top bg-dark flex-nowrap py-1"],
        div![C!["navbar-brand mb-0 h1"], "Fluss"],
        ul![
            C!["navbar-nav mr-auto"],
            li![
                C!["nav-item dropdown"],
                a![
                    //href="#" id="navbarDropdown" role="button" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false"
                    C!["nav-link dropdown-toggle"],
                    "Widgets",
                    attrs![
                        At::Href => "#",
                        At::Id => "navbarDropdown",
                        At::from("role") => "button",
                        At::from("data-toggle") => "dropdown",
                        At::from("aria-haspopup") => "true",
                        At::from("aria-expanded") => "false",
                    ],
                ],
                div![
                    C!["dropdown-menu"],
                    attrs![
                        At::from("aria-labelledby") => "navbarDropdown",
                    ],
                    a![
                        C!["dropdown-item"],
                        attrs![
                            At::Href => "#",
                        ],
                        "🙂 Smiley",
                        ev(Ev::Click, |_| Msg::AddWidget),
                    ]
                ],
            ],
        ],
        ul![
            C!["navbar-nav"],
            li![
                C!["nav-item"],
                a![
                    C!["nav-link"],
                    attrs![
                        At::Href => "https://github.com/cedeber/fluss",
                        At::Target => "_blank",
                        At::Rel => "noopener",
                    ],
                    i![C!["fab fa-github fa-lg"]],
                ],
            ],
        ],
    ]
}
