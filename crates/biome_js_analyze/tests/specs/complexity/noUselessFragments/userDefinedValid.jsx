/* should not generate diagnostics */
function Fragment() {}
let React = { Fragment };
<>
    <Fragment>test</Fragment>
    <React.Fragment>test</React.Fragment>
</>
