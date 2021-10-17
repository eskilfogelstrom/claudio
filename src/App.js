import React from 'react';
import {
  MemoryRouter as Router,
  Link,
  Switch,
  Route,
  Redirect,
  useHistory,
  useLocation
} from 'react-router-dom';

import Return from './components/Return';
import Send from './components/Send';

import './App.css';

function App() {
  return (
    <div className="app">
      <div className="app-menu">
        <Link to="/return">Return</Link>
        <Link to="/send">Send</Link>
      </div>
      <div className="app-content">
        <Switch>
          <Route path="/return" component={Return} />
          <Route path="/send" component={Send} />
          <Redirect from="/" to="/return" />
        </Switch>
      </div>
    </div>
  );
}

export default App;
