import { configureStore } from "@reduxjs/toolkit";
import defaultReducer from './slice';

export default configureStore({
    reducer: {
        default: defaultReducer,
    },
});