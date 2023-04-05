var React = require("react");
module.exports = class extends React.Component {
    render() {
        return <div>

                {false ? <span>IfBlock</span> : null}

            </div>;
    }
};
