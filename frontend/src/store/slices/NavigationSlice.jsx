import { createSlice } from "@reduxjs/toolkit";

export const pagesEnums = {
    main: "main",
    calendar: "calendar",
    settings: "settings",
    login: "login"
}

const initialState =  {
    value: pagesEnums.main
};

const pagesSlice = createSlice({
    name: "pages",
    initialState,
    reducers: {
        changePage: (state, { payload }) => {
            state.value = payload;
        }
    }
});

export const { changePage } = pagesSlice.actions;
export default pagesSlice.reducer;