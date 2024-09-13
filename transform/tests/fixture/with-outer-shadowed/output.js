var React = require("react");

module.exports = class extends React.Component {
    render() {
        var foo = "variable";
        return <div>
                {(function(foo) {
            return <span>{foo}</span>;
        }).call(this, "attribute")}
            </div>;
    }
};
