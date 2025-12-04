import React from 'react';

export class Component extends React.Component {
  render() {
    return (
      <For each="item" index="index" of={this.props.list}>
        <div>
          {this.renderContent({
            index,
            item,
          })}
          {this.renderContent2(item)}
          {(() => {
            const item = index + 1;
            return [item]
          })()}
          {(() => {
            // should be ignored
            this.obj.item;
            class A {
              item() {}
            }
            class B {
              item = 1;
            }

            // should be same ctxt
            this.obj[item];
            const b = item + 1;
            const c = { [item]: 2 };
            return [new A(), new B(), b, c];
          })()}
        </div>
      </For>
    );
  }
}
