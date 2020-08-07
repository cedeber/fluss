export type RectGeometry = {
  x: number;
  y: number;
  width: number;
  height: number;
};

export type Point2 = [number, number]; // [x, y]

export type Widget = {
  name: string;
  uuid: string;
  geometry: RectGeometry;
  initial_geometry: RectGeometry;
  visible: boolean;
};

export type Cursor = {
  active_anchor: "None" | "TopRight" | "TopLeft" | "BottomRight" | "BottomLeft";
  down_start_position: Point2;
  is_active: boolean;
  position: Point2;
  previous_position: Point2;
};

export type WasmSettings = {
  keep_ratio: boolean;
};

export type AppSettings = {
  hideUserInterface: boolean;
};

export type UiGlobalState = {
  cursor: Cursor;
  canvas_geometry: RectGeometry;
  active_widget_uuid: string | null;
  pointer_widget_uuid: string | null;
  settings: WasmSettings;
};

export type Api = {
  activate_events: Function;
  add_widget: Function;
  update_widget: Function;
  select_widget: Function;
  hover_widget: Function;
  toggle_visibility_widget: Function;
  delete_widget: Function;
  swap_widget: Function;
  update_settings: Function;
};
