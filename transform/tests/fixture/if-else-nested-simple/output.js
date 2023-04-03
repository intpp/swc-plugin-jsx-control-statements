var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>

                {this.props.condition !== "world" ? true ? <p>world</p> : <p>Hello</p> : null}

            </div>;
    }
};
