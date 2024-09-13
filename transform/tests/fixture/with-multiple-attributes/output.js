var React = require("react");

module.exports = class extends React.Component {
    render() {
        return <div>
                {(function(attr1, attr2, attr3) {
            return <span>{attr1 + attr2 + attr3}</span>;
        }).call(this, "value1", "value2", "value3")}
            </div>;
    }
};
