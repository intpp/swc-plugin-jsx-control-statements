var React = require("react");
module.exports = class extends React.Component {
    render() {
        const { when, ...otherProps } = this.props;
        return <div>
        {when ? [
            <span key="0">When1</span>,
            <span {...otherProps} key="1">When2</span>
        ] : [
            <span key="0">Other1</span>,
            <span key="1">Other2</span>
        ]}
      </div>;
    }
};