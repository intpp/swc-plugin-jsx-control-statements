var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>

        {this.props.outerWhen ? <><span>test</span>{this.props.innerWhen ? <span>When-When</span> : <span>When-Otherwise</span>}</> : <><span>test</span>{this.props.innerWhen ? <span>Otherwise-When</span> : <span>Otherwise-Otherwise</span>}</>}

      </div>;
    }
};
