import React from 'react';

const SuperComponent = ({ opened }) => (
    <If condition={opened}>
        <div>Hello</div>
        <div>World</div>
    </If>
);

export default SuperComponent;
