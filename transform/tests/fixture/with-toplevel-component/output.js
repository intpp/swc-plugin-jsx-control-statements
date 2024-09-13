var React = require("react");

module.exports = class extends React.Component {
    render() {
        return (function(attr) {
                return <span>{attr}</span>;
            }).call(this, "value");
    }
};
