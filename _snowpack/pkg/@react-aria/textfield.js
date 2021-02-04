import { h as useId, i as useLabels, f as filterDOMProps, m as mergeProps, _ as _extends } from '../common/module-9a1491c0.js';
import { u as useFocusable } from '../common/module-24c545cc.js';
import '../common/index-e66f0a38.js';
import '../common/clsx.m-e1755476.js';

/**
 * Provides the accessibility implementation for labels and their associated elements.
 * Labels provide context for user inputs.
 * @param props - The props for labels and fields.
 */
function useLabel(props) {
  let {
    id,
    label,
    'aria-labelledby': ariaLabelledby,
    'aria-label': ariaLabel,
    labelElementType = 'label'
  } = props;
  id = useId(id);
  let labelId = useId();
  let labelProps = {};

  if (label) {
    ariaLabelledby = ariaLabelledby ? ariaLabelledby + " " + labelId : labelId;
    labelProps = {
      id: labelId,
      htmlFor: labelElementType === 'label' ? id : undefined
    };
  } else if (!ariaLabelledby && !ariaLabel) {
    console.warn('If you do not provide a visible label, you must specify an aria-label or aria-labelledby attribute for accessibility');
  }

  let fieldProps = useLabels({
    id,
    'aria-label': ariaLabel,
    'aria-labelledby': ariaLabelledby
  });
  return {
    labelProps,
    fieldProps
  };
}

/**
 * Provides the behavior and accessibility implementation for a text field.
 * @param props - Props for the text field.
 * @param ref - Ref to the HTML input or textarea element.
 */
function useTextField(props, ref) {
  let {
    inputElementType = 'input',
    isDisabled = false,
    isRequired = false,
    isReadOnly = false,
    validationState,
    type = 'text',
    onChange: _onChange = () => {}
  } = props;
  let {
    focusableProps
  } = useFocusable(props, ref);
  let {
    labelProps,
    fieldProps
  } = useLabel(props);
  let domProps = filterDOMProps(props, {
    labelable: true
  });
  const inputOnlyProps = {
    type,
    pattern: props.pattern
  };
  return {
    labelProps,
    inputProps: mergeProps(domProps, inputElementType === 'input' && inputOnlyProps, _extends({
      disabled: isDisabled,
      readOnly: isReadOnly,
      'aria-required': isRequired || undefined,
      'aria-invalid': validationState === 'invalid' || undefined,
      'aria-errormessage': props['aria-errormessage'],
      'aria-activedescendant': props['aria-activedescendant'],
      'aria-autocomplete': props['aria-autocomplete'],
      'aria-haspopup': props['aria-haspopup'],
      value: props.value,
      defaultValue: props.value ? undefined : props.defaultValue,
      onChange: e => _onChange(e.target.value),
      autoComplete: props.autoComplete,
      maxLength: props.maxLength,
      minLength: props.minLength,
      name: props.name,
      placeholder: props.placeholder,
      inputMode: props.inputMode,
      // Clipboard events
      onCopy: props.onCopy,
      onCut: props.onCut,
      onPaste: props.onPaste,
      // Composition events
      onCompositionEnd: props.onCompositionEnd,
      onCompositionStart: props.onCompositionStart,
      onCompositionUpdate: props.onCompositionUpdate,
      // Selection events
      onSelect: props.onSelect,
      // Input events
      onBeforeInput: props.onBeforeInput,
      onInput: props.onInput
    }, focusableProps, fieldProps))
  };
}

export { useTextField };
