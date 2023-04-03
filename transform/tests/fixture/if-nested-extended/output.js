var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>

                {this.props.condition === "blah" ? <span>

                        {true ? <p>hello</p> : null}



                        {this.props.condition !== "world" ? true ? <p>world</p> : null : null}

                    </span> : null}

            </div>;
    }
};
