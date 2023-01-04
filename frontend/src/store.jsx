import { configureStore } from "@reduxjs/toolkit";
import defaultReducer from './slice';
import pagesReducer from './NavigationSlice';

export default configureStore({
    reducer: {
        default: defaultReducer,
        pages: pagesReducer
    },
});