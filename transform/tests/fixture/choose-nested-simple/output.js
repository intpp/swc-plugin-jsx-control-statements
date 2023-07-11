var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>

        {this.props.outerWhen ? [
            <span key="0">test</span>,
            this.props.innerWhen ? <span key="1">When-When</span> : <span key="1">When-Otherwise</span>
        ] : [
            <span key="0">test</span>,
            this.props.innerWhen ? <span key="1">Otherwise-When</span> : <span key="1">Otherwise-Otherwise</span>
        ]}

      </div>;
    }
};
