import { useDispatch, useSelector } from 'react-redux';
import Header from "./Header/Header"
import MainSection from './MainSection/MainSection';

function App() {
  
  return (
      <div className='mainPage'>
        <Header />
        <MainSection />
      </div>
    );
}

export default App;
