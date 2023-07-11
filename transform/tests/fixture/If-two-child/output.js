import React from 'react';
const SuperComponent = ({ opened  })=>opened ? [
        <div key="0">Hello</div>,
        <div key="1">World</div>
    ] : null;
export default SuperComponent;
