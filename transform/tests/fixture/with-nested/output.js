var React = require("react");

module.exports = class extends React.Component {
    render() {
        return <div>
                {(function(attr1) {
            return (function(attr2) {
                    return <span>{attr1 + attr2}</span>;
                }).call(this, "inner")
        }).call(this, "outer")}
            </div>;
    }
};
