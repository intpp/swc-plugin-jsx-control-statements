var React = require("react");

module.exports = class extends React.Component {
    render() {
        return (
            <div>
                <If condition="">
                    <span>IfBlock</span>
                </If>
            </div>
        );
    }
};
