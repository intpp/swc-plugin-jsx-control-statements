declare type ConditionalTagProps = { condition: boolean, children: React.ReactNode };
declare function Choose(props: { children: React.ReactNode }): any;
declare function When(props: ConditionalTagProps): any;
declare function Otherwise(props: { children: React.ReactNode }): any;
declare function If(props: ConditionalTagProps): any;
