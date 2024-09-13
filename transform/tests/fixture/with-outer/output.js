var React = require("react");

module.exports = class extends React.Component {
    render() {
        var foo = "variable";
        return <div>
                {(function(bar) {
            return <span>{foo + bar}</span>;
        }).call(this, "attribute")}
            </div>;
    }
};
