var React = require("react");
module.exports = class extends React.Component {
    render() {
        return this.props.when1 ? <span>WhenBlock1</span> : <span>OtherwiseBlock</span>;
    }
};
