import { useDispatch, useSelector } from 'react-redux';
import Header from "./Header/Header"
import MainSection from './MainSection/MainSection';

function App() {
  const dispatch = useDispatch();
  const defaultState = useSelector(state => state.default.value);
  
  return (
      <div className='mainPage'>
        <Header />
        <MainSection />
      </div>
    );
}

export default App;
