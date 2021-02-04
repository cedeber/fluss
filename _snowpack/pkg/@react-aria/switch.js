import { m as mergeProps, _ as _extends, u as usePress, f as filterDOMProps } from '../common/module-9a1491c0.js';
import { u as useFocusable } from '../common/module-24c545cc.js';
import '../common/index-e66f0a38.js';
import '../common/clsx.m-e1755476.js';

/**
 * Handles interactions for toggle elements, e.g. Checkboxes and Switches.
 */
function useToggle(props, state, ref) {
  let {
    isDisabled = false,
    isRequired,
    isReadOnly,
    value,
    name,
    children,
    'aria-label': ariaLabel,
    'aria-labelledby': ariaLabelledby,
    validationState = 'valid'
  } = props;

  let onChange = e => {
    // since we spread props on label, onChange will end up there as well as in here.
    // so we have to stop propagation at the lowest level that we care about
    e.stopPropagation();
    state.setSelected(e.target.checked);
  };

  let hasChildren = children != null;
  let hasAriaLabel = ariaLabel != null || ariaLabelledby != null;

  if (!hasChildren && !hasAriaLabel) {
    console.warn('If you do not provide children, you must specify an aria-label for accessibility');
  } // This handles focusing the input on pointer down, which Safari does not do by default.


  let {
    pressProps
  } = usePress({
    isDisabled
  });
  let {
    focusableProps
  } = useFocusable(props, ref);
  let interactions = mergeProps(pressProps, focusableProps);
  let domProps = filterDOMProps(props, {
    labelable: true
  });
  return {
    inputProps: mergeProps(domProps, _extends({
      'aria-invalid': validationState === 'invalid' || undefined,
      'aria-errormessage': props['aria-errormessage'],
      'aria-controls': props['aria-controls'],
      onChange,
      disabled: isDisabled,
      required: isRequired,
      readOnly: isReadOnly,
      value,
      name,
      type: 'checkbox'
    }, interactions))
  };
}

/**
 * Provides the behavior and accessibility implementation for a switch component.
 * A switch is similar to a checkbox, but represents on/off values as opposed to selection.
 * @param props - Props for the switch.
 * @param state - State for the switch, as returned by `useToggleState`.
 * @param ref - Ref to the HTML input element.
 */
function useSwitch(props, state, ref) {
  let {
    inputProps
  } = useToggle(props, state, ref);
  let {
    isSelected
  } = state;
  return {
    inputProps: _extends({}, inputProps, {
      role: 'switch',
      checked: isSelected,
      'aria-checked': isSelected
    })
  };
}

export { useSwitch };
