var React = require("react");

module.exports = class extends React.Component {
    render() {
        return <div>
                {(function(attr) {
            return (function(attr) {
                    return <span>{attr}</span>;
            }).call(this, "inner")
        }).call(this, "outer")}
            </div>;
    }
};
