export default defineAppConfig({
  ui: {
    button: {
      slots: {
        base: 'cursor-pointer text-text-100'
      },
      compoundVariants: [
        {
          color: 'primary',
          variant: 'solid',
          class: 'text-text-100 '
        }
      ]
    },
    checkbox: {
      slots: {
        indicator: 'text-text-100'
      }
    },
    radioGroup: {
      slots: {
        base: 'cursor-pointer'
      }
    }
  }
});
