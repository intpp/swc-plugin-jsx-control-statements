var React = require("react");

module.exports = class extends React.Component {
    render() {
        let test = "test";
        return <div>
                {(function() {
            return <span>{test}</span>;
        }).call(this)}
            </div>;
    }
};
