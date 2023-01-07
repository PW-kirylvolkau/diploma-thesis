import React from 'react';
import {
  Grid,
  TextField,
  Paper,
  Button
} from '@mui/material';
import { useState } from 'react';

export default function Login() {

  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [usernameError, setUsernameError] = useState('');
  const [passwordError, setPasswordError] = useState('');

  function loginClick() {
    if(usernameError.length === 0 && passwordError.length === 0) {
      console.log(`sending to backend: ${username} ${password}`);
    } else {
      console.log("fix your username or password");
    }
  }

  function usernameOnChange(event) {
    event.preventDefault();
    setUsername(event.target.value); 
    isUsernameValid(event.target.value); 
  }

  function isUsernameValid(name) {
    if(name.length < 5) {
      setUsernameError('should have more than 5 characters');
    } else {
      setUsernameError('');
    }
  }

  function passwordOnChange(event) {
    event.preventDefault();
    setPassword(event.target.value);
    isPasswordValid(event.target.value);
  }

  function isPasswordValid(password) {
    if(password.length < 8) {
      setPasswordError('should have more than 8 characters');
    } else {
      setPasswordError('');
    }
  }


  return (
    <div style={{ padding: 30 }}>
      <Paper sx={{
        width: '40%',
        margin: '10vh auto',
        padding: '10vh 0'
      }}>
        <Grid
          container
          spacing={4}
          direction={'column'}
          justify={'center'}
          alignItems={'center'}
        >
          <Grid item xs={12}>
            <TextField 
              label="Username"
              sx={{ width: '20vw' }}
              onChange={usernameOnChange}
              helperText={usernameError}
            />
          </Grid>
          <Grid item xs={12}>
            <TextField 
              label="Password"
              type={'password'}
              sx={{ width: '20vw' }} 
              onChange={passwordOnChange}
              helperText={passwordError}
            />
          </Grid>
          <Grid item xs={12}>
            <Button fullWidth variant='outlined' onClick={loginClick}
              sx={{ width: '20vw' }}> Login </Button>
          </Grid>
        </Grid>
      </Paper>
    </div>
  );
};