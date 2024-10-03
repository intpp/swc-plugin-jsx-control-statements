var React = require("react");
module.exports = class extends React.Component {
    render() {
        this.test = "test";
        return <div>
        {this.props.items.map((item, index)=><span key={item}>{item + this.test}</span>, this)}
      </div>;
    }
};
