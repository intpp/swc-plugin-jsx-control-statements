# 🦀 JSX control statements for swc

Original idea: [babel-plugin-jsx-control-statements](https://github.com/AlexGilleran/jsx-control-statements)

## Installation

**pnpm** (recommended):

```shell
pnpm i -D swc-plugin-jsx-control-statements
```

or **yarn**

```shell
yarn add -D swc-plugin-jsx-control-statements
```

## Configure `swc`

In your SWC config, you have to add to [`jsc.experimental.plugins`](https://swc.rs/docs/configuration/compilation#jscexperimentalplugins) - `['swc-plugin-jsx-control-statements', {}]`, like in the following code:

```javascript
jsc: {
    experimental: {
        plugins: [
            ['swc-plugin-jsx-control-statements', {}],
        ],
    },
},
```

## Usage

### `<If>` tag

```jsx
import React from 'react';

const Greeting = () => {
  const [closed, setClosed] = useState(false);
  
  return (
      <>
          <If condition={!closed}>
              Hello,
          </If>
          World
          <If condition={!closed}>
            <button onClick={() => setClosed(true)}>Close</button>
          </If>
      </>
  )
};
```


### `<Choose>` tag (if - else if - else condition)

```jsx
import React from 'react';

const Greeting = () => {
  const [closed, setClosed] = useState(false);
  
  return (
      <>
          <Choose>
              <When condition={!closed}>
                  Hello,
              </When>
              <Otherwise>
                  Bye,
              </Otherwise>
          </Choose>
          World
          <If condition={!closed}>
            <button onClick={() => setClosed(true)}>Close</button>
          </If>
      </>
  )
};
```

### For Tag

Define `<For>` like so:

```jsx
  // you must provide the key attribute yourself
  <For each="item" of={ this.props.items }>
    <span key={ item.id }>{ item.title }</span>
  </For>

  // using the index as key attribute is not stable if the array changes
  <For each="item" index="idx" of={ [1,2,3] }>
    <span key={ idx }>{ item }</span>
    <span key={ idx + '_2' }>Static Text</span>
  </For>
```

### With Tag

Used to assign values to local variables:

```javascript
// simple
<With foo={ 47 } bar={ 'test' }>
  <span>{ foo }</span>
  <span>{ bar }</span>
</With>

// nested
<With foo={ 47 }>
  <With bar={ 'test' }>
    <span>{ foo }</span>
    <span>{ bar }</span>
  </With>
</With>
```