// @ts-expect-error A conditional requires a boolean.
<If condition="yes">invalid</If>;

// @ts-expect-error Else is a self-closing separator.
<Else>invalid</Else>;

// @ts-expect-error The callback item is inferred from the array.
<For of={[1, 2]} body={item => item.toUpperCase()} />;
