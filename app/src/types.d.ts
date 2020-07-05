declare interface Window {
  update_app_state: Function;
}

declare module "*.vue" {
  import Vue from "vue";
  export default Vue;
}
