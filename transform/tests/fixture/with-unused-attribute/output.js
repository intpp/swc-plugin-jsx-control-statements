var React = require("react");

module.exports = class extends React.Component {
    render() {
        return <div>
                {(function(attr1, attr2) {
                return <span>{attr1}</span>;
            }).call(this, "used", "unused")}
            </div>
    }
};
