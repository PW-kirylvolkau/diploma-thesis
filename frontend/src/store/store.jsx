import { configureStore } from "@reduxjs/toolkit";
import defaultReducer from './slices/DefaultSlice';
import pagesReducer from './slices/NavigationSlice';

export default configureStore({
    reducer: {
        default: defaultReducer,
        pages: pagesReducer
    },
});