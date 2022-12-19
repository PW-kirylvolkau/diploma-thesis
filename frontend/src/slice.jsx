import { createSlice } from "@reduxjs/toolkit";

const initialState =  {
    value: "StateString"
};

const defaultSlice = createSlice({
    name: "default",
    initialState,
    reducers: {
        changeState: (state, { payload }) => {
            state = payload;
        }
    }
});

export const { changeState } = defaultSlice.actions;
export default defaultSlice.reducer;