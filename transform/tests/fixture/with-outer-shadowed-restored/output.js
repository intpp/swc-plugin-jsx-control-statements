var React = require("react");

module.exports = class extends React.Component {
    render() {
        var foo = "variable";
        return <div>
                <span>{foo}</span>
                {(function(foo) {
            return <span>{foo}</span>;
        }).call(this, "attribute")}
                <span>{foo}</span>
            </div>;
    }
};
