import { useDispatch, useSelector } from 'react-redux';

function App() {
  const dispatch = useDispatch();
  const defaultState = useSelector(state => state.default.value);
  
  return (
    <div>
      <h1>{defaultState}</h1>
    </div>
    );
}

export default App;
