import { useSelector } from "react-redux";
import { pagesEnums } from "../store/slices/NavigationSlice";
import "./MainSection.css";
import Calendar from "./Pages/Calendar/Calendar";
import Login from "./Pages/Login/Login";
import Settings from "./Pages/Settings/Settings";

function getPage() {
    const page = useSelector(state => state.pages.value);


    switch(page) {
        case pagesEnums.calendar:
            return <Calendar />;
        case pagesEnums.login:
            return <Login />;
        case pagesEnums.settings: 
            return <Settings />;
        default: 
            return <h1>No Page</h1>;
    }
}

export default function MainSection() {
    return (<div className="main-section">
        {getPage()}
    </div>);
}