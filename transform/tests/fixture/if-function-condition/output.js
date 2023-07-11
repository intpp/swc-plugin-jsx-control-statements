import React from 'react';
export default (({ shouldDisplay })=>{
    return <div>

            {shouldDisplay() ? "Tada!" : null}

        </div>;
});
