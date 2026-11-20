import type { ReactNode } from "react";

const names = ["Ada", "Grace"] as const;
const name = "outer";
declare const position: number;
declare const active: boolean;
declare const label: ReactNode;

export function Example({ enabled }: { enabled: boolean }) {
    return <main>
        <If condition={enabled}>on<Else />off</If>
        <Choose>
            <When condition={enabled}>yes</When>
            <Otherwise>no</Otherwise>
        </Choose>
        <For each="name" index="position" of={names}>
            <b key={name}>{position}{name}</b>
        </For>
        <For of={names} body={(item, index) => <i key={item}>{item.toUpperCase()}{index.toFixed()}</i>} />
        <With name="inner">
            <span>{name}</span>
            <With name={name.toUpperCase()}><span key={name}>{name}</span></With>
            <span>{names.map(name => name.toLowerCase()).join(",")}</span>
        </With>
        <span>{name}</span>
    </main>;
}

export function unicode() {
    return <If condition={true} key="&#xD800;">
        <b>first</b>
        <b>second</b>
        &#xD800;
    </If>;
}

export function IndexList() {
    const _ = "outer";
    return <With active label={<em key="index">index</em>}>
        <If condition={active}>{label}</If>
        <For of={names} index="position"><i key={position}>{_}{position}</i></For>
        <For of={names}>!</For>
    </With>;
}
