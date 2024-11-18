import ReactA, { useState } from 'react'

const Component = () => {
  ReactA.useEffect(() => {
    console.log("Hello");
  }, []);

  return <div>Hello</div>
}

export default Component;