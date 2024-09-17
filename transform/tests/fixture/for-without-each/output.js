var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>
        {[
            1,
            2,
            3
        ].map(function(_, _) {
            return "ABC";
        }, this)}
      </div>;
    }
};
