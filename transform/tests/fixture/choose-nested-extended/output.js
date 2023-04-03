var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>

        {this.props.outerWhen ? this.props.innerWhen ? <span>When-When</span> : <span>When-Otherwise</span> : "Otherwise"}

      </div>;
    }
};
