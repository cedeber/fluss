import { a as useFocusVisible, b as useFocus, c as useFocusWithin, m as mergeProps, _ as _extends, d as _objectWithoutPropertiesLoose, e as useKeyboard } from './module-9a1491c0.js';
import { _ as __pika_web_default_export_for_treeshaking__ } from './clsx.m-e1755476.js';
import { r as react } from './index-e66f0a38.js';

react.createContext(null);

/**
 * Determines whether a focus ring should be shown to indicate keyboard focus.
 * Focus rings are visible only when the user is interacting with a keyboard,
 * not with a mouse, touch, or other input methods.
 */
function useFocusRing(props) {
  if (props === void 0) {
    props = {};
  }

  let {
    within
  } = props;
  let [isFocused, setFocused] = react.useState(false);
  let [isFocusWithin, setFocusWithin] = react.useState(false);
  let {
    isFocusVisible
  } = useFocusVisible(props);
  let {
    focusProps
  } = useFocus({
    isDisabled: within,
    onFocusChange: setFocused
  });
  let {
    focusWithinProps
  } = useFocusWithin({
    isDisabled: !within,
    onFocusWithinChange: setFocusWithin
  });
  return {
    isFocused: within ? isFocusWithin : isFocused,
    isFocusVisible: (within ? isFocusWithin : isFocused) && isFocusVisible,
    focusProps: within ? focusWithinProps : focusProps
  };
}

/**
 * A utility component that applies a CSS class when an element has keyboard focus.
 * Focus rings are visible only when the user is interacting with a keyboard,
 * not with a mouse, touch, or other input methods.
 */
function FocusRing(props) {
  let {
    children,
    focusClass,
    focusRingClass
  } = props;
  let {
    isFocused,
    isFocusVisible,
    focusProps
  } = useFocusRing(props);

  let child = react.Children.only(children);

  return react.cloneElement(child, mergeProps(child.props, _extends({}, focusProps, {
    className: __pika_web_default_export_for_treeshaking__({
      [focusClass || '']: isFocused,
      [focusRingClass || '']: isFocusVisible
    })
  })));
}

let $e11539c8317b2d21639df611cb5658f$var$FocusableContext = react.createContext(null);

function $e11539c8317b2d21639df611cb5658f$var$useFocusableContext(ref) {
  let context = react.useContext($e11539c8317b2d21639df611cb5658f$var$FocusableContext) || {};
  react.useEffect(() => {
    if (context && context.ref) {
      context.ref.current = ref.current;
      return () => {
        context.ref.current = null;
      };
    }
  }, [context, ref]);
  return context;
}
/**
 * Provides DOM props to the nearest focusable child.
 */


function $e11539c8317b2d21639df611cb5658f$var$FocusableProvider(props, ref) {
  let {
    children
  } = props,
      otherProps = _objectWithoutPropertiesLoose(props, ["children"]);

  let context = _extends({}, otherProps, {
    ref
  });

  return /*#__PURE__*/react.createElement($e11539c8317b2d21639df611cb5658f$var$FocusableContext.Provider, {
    value: context
  }, children);
}

react.forwardRef($e11539c8317b2d21639df611cb5658f$var$FocusableProvider);

/**
 * Used to make an element focusable and capable of auto focus.
 */
function useFocusable(props, domRef) {
  let {
    focusProps
  } = useFocus(props);
  let {
    keyboardProps
  } = useKeyboard(props);
  let interactions = mergeProps(focusProps, keyboardProps);
  let domProps = $e11539c8317b2d21639df611cb5658f$var$useFocusableContext(domRef);
  let interactionProps = props.isDisabled ? {} : domProps;
  react.useEffect(() => {
    if (props.autoFocus && domRef.current) {
      domRef.current.focus();
    }
  }, [props.autoFocus, domRef]);
  return {
    focusableProps: mergeProps(_extends({}, interactions, {
      tabIndex: props.excludeFromTabOrder && !props.isDisabled ? -1 : undefined
    }), interactionProps)
  };
}

export { FocusRing as F, useFocusRing as a, useFocusable as u };
