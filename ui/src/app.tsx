import React from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import Debugger from './routes/debugger';
import NotFoundPage from './routes/notfound';
import GlobalStyle from './components/global-style';

const App: React.FC = () => {
    return (
        <div id="root">
            <GlobalStyle />
            <BrowserRouter>
                <Routes>
                <Route path="/" element={<Debugger/>} />
                <Route path="*" element={<NotFoundPage/>} />
                </Routes>
            </BrowserRouter>
        </div>
    );
};

export default App;
