var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>

        {true ? [
            this.props.type === "a" ? <span key="0">

              Blah A

            </span> : this.props.type === "b" ? <span key="0">

              Blah B

            </span> : null,
            this.props.type === "a" ? <span key="1">

              Blah C

            </span> : this.props.type === "b" ? <span key="1">

              Blah D

            </span> : null
        ] : null}

      </div>;
    }
};
