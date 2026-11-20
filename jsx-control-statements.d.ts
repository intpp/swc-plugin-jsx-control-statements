import type { ReactElement, ReactNode } from "react";

type ForProps<T> = { of: Iterable<T> } & (
    | { body: (item: T, index: number) => ReactNode; children?: never; each?: never; index?: never }
    | { body?: never; children?: ReactNode; each?: string; index?: string }
);

declare global {
    type ConditionalTagProps = { condition: boolean; children?: ReactNode };

    function If(props: ConditionalTagProps): ReactElement;
    function Else(props: { children?: never }): ReactElement;
    function Choose(props: { children: ReactNode }): ReactElement;
    function When(props: ConditionalTagProps): ReactElement;
    function Otherwise(props: { children: ReactNode }): ReactElement;

    function For<T>(props: ForProps<T>): ReactElement;

    function With(props: { [name: string]: unknown; children?: ReactNode }): ReactElement;
}
