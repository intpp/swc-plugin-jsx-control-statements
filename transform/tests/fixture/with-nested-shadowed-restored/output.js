var React = require("react");

module.exports = class extends React.Component {
    render() {
        return <div>
                {(function(attr) {
            return [
                <span key="0">{attr}</span>,
                (function(attr) {
                    return <span>{attr}</span>;
                }).call(this, "inner"),
                <span key="2">{attr}</span>
           ];
        }).call(this, "outer")}
            </div>;
    }
};
