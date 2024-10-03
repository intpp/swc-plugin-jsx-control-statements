var React = require("react");

module.exports = class extends React.Component {
    render() {
        this.foo = "outer";
        return <div>
                {(function(foo) {
            return <span>{foo + this.foo}</span>;
        }).call(this, "attribute")}
            </div>;
    }
};
