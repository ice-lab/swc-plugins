import * as ReactA from 'react'

const Component = () => {
  ReactA.useEffect(() => {
    console.log("Hello");
  }, []);

  return <div>Hello</div>
}

export default Component;