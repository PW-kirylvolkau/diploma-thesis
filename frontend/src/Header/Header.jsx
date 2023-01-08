import  "./Header.css";
import Button from "@mui/material/Button";
import LoginIcon from '@mui/icons-material/Login';
import CalendarMonthIcon from '@mui/icons-material/CalendarMonth';
import SettingsIcon from '@mui/icons-material/Settings';
import { Avatar, ButtonGroup } from "@mui/material";
import { useDispatch } from "react-redux";
import { changePage, pagesEnums } from "../store/slices/NavigationSlice";

export default function Header() {
    const dispatch = useDispatch();

    return (
        <nav>
            <ButtonGroup size="small" sx={{ float: "left" }}>
                <Button 
                    className="button" 
                    sx={{ width: '300px', height: '8vh' }}
                    onClick={() => {dispatch(changePage(pagesEnums.main));}}
                >
                    <div className="label">LOGO</div>
                </Button>
            </ButtonGroup>
            <ButtonGroup size="small" sx={{ float: "right" }}>
                <Button 
                    className="button" 
                    sx={{ width: '150px', height: '8vh' }}
                    onClick={() => {dispatch(changePage(pagesEnums.calendar));}}
                >
                    <CalendarMonthIcon fontSize="medium" sx={{paddingRight: 1}} />
                    <div className="label">Calendar</div>
                </Button>
                <Button 
                    className="button" 
                    sx={{ width: '150px' }}
                    onClick={() => {dispatch(changePage(pagesEnums.settings));}}
                >
                    <SettingsIcon fontSize="medium" sx={{paddingRight: 1}} />
                    <div className="label">Settings</div>
                </Button>
                <Button 
                    className="button" 
                    sx={{ width: '150px' }}
                    onClick={() => {dispatch(changePage(pagesEnums.login));}}
                >
                    <LoginIcon fontSize="medium" sx={{paddingRight: 1}}/>
                    <div className="label">LOGIN</div>
                </Button>
                <Avatar sx={{
                    width: '20px',
                    height: '20px',
                    padding: '10px',
                    marginLeft: 4,
                    marginRight: 2,
                    marginTop: 1,
                    marginBottom: 1,
                    bgcolor: "#0081C9"
                }}>?</Avatar>
            </ButtonGroup>
        </nav>);
}