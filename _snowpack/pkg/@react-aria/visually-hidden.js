import { d as _objectWithoutPropertiesLoose, _ as _extends, m as mergeProps, b as useFocus } from '../common/module-9a1491c0.js';
import { r as react } from '../common/index-e66f0a38.js';
import '../common/clsx.m-e1755476.js';

const $f431c2c11cc559fa3c5864caafbcfd19$var$styles = {
  border: 0,
  clip: 'rect(0 0 0 0)',
  clipPath: 'inset(50%)',
  height: 1,
  margin: '0 -1px -1px 0',
  overflow: 'hidden',
  padding: 0,
  position: 'absolute',
  width: 1,
  whiteSpace: 'nowrap'
};

/**
 * Provides props for an element that hides its children visually
 * but keeps content visible to assistive technology.
 */
function useVisuallyHidden(props) {
  if (props === void 0) {
    props = {};
  }

  let {
    style,
    isFocusable
  } = props;
  let [isFocused, setFocused] = react.useState(false);
  let {
    focusProps
  } = useFocus({
    isDisabled: !isFocusable,
    onFocusChange: setFocused
  }); // If focused, don't hide the element.

  let combinedStyles = react.useMemo(() => {
    if (isFocused) {
      return style;
    } else if (style) {
      return _extends({}, $f431c2c11cc559fa3c5864caafbcfd19$var$styles, style);
    } else {
      return $f431c2c11cc559fa3c5864caafbcfd19$var$styles;
    }
  }, [isFocused]);
  return {
    visuallyHiddenProps: _extends({}, focusProps, {
      style: combinedStyles
    })
  };
}
/**
 * VisuallyHidden hides its children visually, while keeping content visible
 * to screen readers.
 */

function VisuallyHidden(props) {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  let {
    children,
    elementType: Element = 'div'
  } = props,
      otherProps = _objectWithoutPropertiesLoose(props, ["children", "elementType", "isFocusable", "style"]);

  let {
    visuallyHiddenProps
  } = useVisuallyHidden(props);
  return /*#__PURE__*/react.createElement(Element, mergeProps(otherProps, visuallyHiddenProps), children);
}

export { VisuallyHidden };
