var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>

        {true ? <>{this.props.type === "a" ? <span>

              Blah A

            </span> : this.props.type === "b" ? <span>

              Blah B

            </span> : null}{this.props.type === "a" ? <span>

              Blah C

            </span> : this.props.type === "b" ? <span>

              Blah D

            </span> : null}</> : null}

      </div>;
    }
};
