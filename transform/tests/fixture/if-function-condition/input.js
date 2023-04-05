import React from 'react';

export default ({ shouldDisplay }) => {
    return (
        <div>
            <If condition={shouldDisplay()}>
                Tada!
            </If>
        </div>
    )
}
