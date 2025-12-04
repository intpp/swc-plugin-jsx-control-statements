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
module.exports.A = ()=>{
    const _1 = 1;
    return <div>
      {[
        1,
        2,
        3
    ].map(function(_, _) {
        return [
            "ABC",
            _1
        ];
    }, this)}
    </div>;
};