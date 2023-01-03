import  "./Header.css";
import Button from "@mui/material/Button";
import LoginIcon from '@mui/icons-material/Login';
import CalendarMonthIcon from '@mui/icons-material/CalendarMonth';
import SettingsIcon from '@mui/icons-material/Settings';
import { Avatar, ButtonGroup } from "@mui/material";

export default function Header() {

    return (
        <nav>
            <ButtonGroup size="small" sx={{float: "right"}}>
                <Button 
                    className="button" 
                    sx={{ width: '150px' }}
                >
                    <CalendarMonthIcon fontSize="medium" sx={{paddingRight: 1}} />
                    <div className="label">Calendar</div>
                </Button>
                <Button 
                    className="button" 
                    sx={{ width: '150px' }}
                >
                    <SettingsIcon fontSize="medium" sx={{paddingRight: 1}} />
                    <div className="label">Settings</div>
                </Button>
                <Button 
                    className="button" 
                    sx={{ width: '150px' }}
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
                    marginBottom: 1
                }}>P</Avatar>
            </ButtonGroup>
        </nav>);
}