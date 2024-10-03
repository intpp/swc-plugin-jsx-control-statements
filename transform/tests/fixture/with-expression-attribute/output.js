var React = require("react");

module.exports = class extends React.Component {
    render() {
        return <div>
                {(function(attr) {
            return <span>{attr()}</span>;
        }).call(this, ()=>"expr" + "ession")}
            </div>;
    }
};
