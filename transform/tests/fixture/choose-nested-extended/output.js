var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>

        {this.props.outerWhen ? this.props.innerWhen ? <span key="blah">When-When</span> : <span key="blah">When-Otherwise</span> : "Otherwise"}

      </div>;
    }
};
