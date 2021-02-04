import { r as react } from '../common/index-e66f0a38.js';

function useControlledState(value, defaultValue, onChange) {
  let [stateValue, setStateValue] = react.useState(value || defaultValue);
  let ref = react.useRef(value !== undefined);
  let wasControlled = ref.current;
  let isControlled = value !== undefined; // Internal state reference for useCallback

  let stateRef = react.useRef(stateValue);

  if (wasControlled !== isControlled) {
    console.warn("WARN: A component changed from " + (wasControlled ? 'controlled' : 'uncontrolled') + " to " + (isControlled ? 'controlled' : 'uncontrolled') + ".");
  }

  ref.current = isControlled;
  let setValue = react.useCallback(function (value) {
    for (var _len = arguments.length, args = new Array(_len > 1 ? _len - 1 : 0), _key = 1; _key < _len; _key++) {
      args[_key - 1] = arguments[_key];
    }

    let onChangeCaller = function onChangeCaller(value) {
      if (onChange) {
        if (stateRef.current !== value) {
          for (var _len2 = arguments.length, onChangeArgs = new Array(_len2 > 1 ? _len2 - 1 : 0), _key2 = 1; _key2 < _len2; _key2++) {
            onChangeArgs[_key2 - 1] = arguments[_key2];
          }

          onChange(value, ...onChangeArgs);
        }
      }

      if (!isControlled) {
        stateRef.current = value;
      }
    };

    if (typeof value === 'function') {
      // this supports functional updates https://reactjs.org/docs/hooks-reference.html#functional-updates
      // when someone using useControlledState calls setControlledState(myFunc)
      // this will call our useState setState with a function as well which invokes myFunc and calls onChange with the value from myFunc
      // if we're in an uncontrolled state, then we also return the value of myFunc which to setState looks as though it was just called with myFunc from the beginning
      // otherwise we just return the controlled value, which won't cause a rerender because React knows to bail out when the value is the same
      let updateFunction = function updateFunction(oldValue) {
        for (var _len3 = arguments.length, functionArgs = new Array(_len3 > 1 ? _len3 - 1 : 0), _key3 = 1; _key3 < _len3; _key3++) {
          functionArgs[_key3 - 1] = arguments[_key3];
        }

        let interceptedValue = value(isControlled ? stateRef.current : oldValue, ...functionArgs);
        onChangeCaller(interceptedValue, ...args);

        if (!isControlled) {
          return interceptedValue;
        }

        return oldValue;
      };

      setStateValue(updateFunction);
    } else {
      if (!isControlled) {
        setStateValue(value);
      }

      onChangeCaller(value, ...args);
    }
  }, [isControlled, onChange]); // If a controlled component's value prop changes, we need to update stateRef

  if (isControlled) {
    stateRef.current = value;
  } else {
    value = stateValue;
  }

  return [value, setValue];
}

/**
 * Provides state management for toggle components like checkboxes and switches.
 */
function useToggleState(props) {
  if (props === void 0) {
    props = {};
  }

  let {
    isReadOnly,
    onChange
  } = props; // have to provide an empty function so useControlledState doesn't throw a fit
  // can't use useControlledState's prop calling because we need the event object from the change

  let [isSelected, setSelected] = useControlledState(props.isSelected, props.defaultSelected || false, () => {});

  function updateSelected(value) {
    if (!isReadOnly) {
      setSelected(value);

      if (onChange) {
        onChange(value);
      }
    }
  }

  function toggleState() {
    if (!isReadOnly) {
      setSelected(prev => {
        let newVal = !prev;

        if (onChange) {
          onChange(newVal);
        }

        return newVal;
      });
    }
  }

  return {
    isSelected,
    setSelected: updateSelected,
    toggle: toggleState
  };
}

export { useToggleState };
