var React = require("react");

module.exports = class extends React.Component {
    render() {
        return <div>
                {(function(attr) {
            return [
                "text child",
                attr
            ];
        }).call(this, "value")}
            </div>;
    }
};
