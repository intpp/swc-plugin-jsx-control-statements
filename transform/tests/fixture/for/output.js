var React = require("react");
module.exports = class extends React.Component {
    render() {
        this.test = "test";
        const item = 1;
        return <div>
        {this.props.items.map(function(item, _){
            return <span key={item}>{item + this.test}</span>;
        }, this)}
      </div>;
    }
};
