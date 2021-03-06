import { readCookie, writeCookie } from "./cookie-portal";

export const storage = getProxy(localStorage);
export const session = getProxy(sessionStorage);

export default storage;

function getProxy(webStorage: Storage): any {
  const hasStorage = hasStorageSupport(webStorage);

  return new Proxy(
    {},
    {
      set(_obj, prop, value) {
        const stringifyValue = JSON.stringify(value);
        hasStorage
          ? webStorage.setItem(prop.toString(), stringifyValue)
          : writeCookie(prop.toString(), stringifyValue);

        return true;
      },
      get(_obj, prop) {
        const value = hasStorage
          ? webStorage.getItem(prop.toString())
          : readCookie(prop.toString());
        return JSON.parse(String(value));
      },
      has(obj, prop) {
        return typeof Reflect.get(obj, prop) === "string";
      },
      deleteProperty(_obj, prop) {
        hasStorage
          ? webStorage.removeItem(prop.toString())
          : writeCookie(prop.toString(), "", { expires: -1 });

        return true;
      },
    },
  );
}

/**
 * Whether the current browser supports local storage as a way of storing data
 */
function hasStorageSupport(webStorage: Storage): boolean {
  try {
    webStorage.setItem("__storage__", "foo");
    webStorage.removeItem("__storage__");

    return true;
  } catch (e) {
    return false;
  }
}
