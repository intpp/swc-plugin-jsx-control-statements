var React = require("react");

module.exports = class extends React.Component {
    render() {
        return <div>
                {(function(attr) {
            return null;
        }).call(this, "value")}
            </div>;
    }
};
