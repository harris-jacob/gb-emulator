import { FunctionalComponent, h } from 'preact';
import { Route, Router } from 'preact-router';
import Debugger from '../routes/debugger';
import NotFoundPage from '../routes/notfound';
import GlobalStyle from './global-style';

const App: FunctionalComponent = () => {

    return (
        <div id="preact_root">
            <GlobalStyle />
            <Router>
                <Route path="/" component={Debugger} />
                <NotFoundPage default />
            </Router>
        </div>
    );
};

export default App;
