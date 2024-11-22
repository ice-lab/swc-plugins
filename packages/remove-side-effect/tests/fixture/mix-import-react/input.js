import React, { useLayoutEffect } from 'react'
import { useEffect } from 'react'

const Component = () => {
  React.useEffect(() => {
    console.log("React.useEffect");
  }, []);

  useEffect(() => {
    console.log("useEffect");
  }, []);

  useLayoutEffect(() => {
    console.log("useLayoutEffect");
  }, []);

  return <div>Hello</div>
}

export default Component;