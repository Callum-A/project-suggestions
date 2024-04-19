import Nav from 'react-bootstrap/Nav';
import Navbar from 'react-bootstrap/Navbar';
import { useAppState } from '../store';
import { Link } from 'react-router-dom';

const GOOGLE_SCOPES = (process.env.REACT_APP_GOOGLE_SCOPES as string)
  .split(',')
  .join(' ');
const GOOGLE_CLIENT_ID = process.env.REACT_APP_GOOGLE_CLIENT_ID as string;
const GOOGLE_REDIRECT_URI = process.env.REACT_APP_GOOGLE_REDIRECT_URI as string;
const params = new URLSearchParams();
params.append('client_id', GOOGLE_CLIENT_ID);
params.append('redirect_uri', GOOGLE_REDIRECT_URI);
params.append('response_type', 'code');
params.append('scope', GOOGLE_SCOPES);
params.append('state', '1');
const queryString = params.toString();
const GOOGLE_URL = `https://accounts.google.com/o/oauth2/v2/auth?${queryString}`;

function NavBar() {
  const isLoggedIn = useAppState((state) => state.isLoggedIn);
  const logout = useAppState((state) => state.logout);

  const onClickLogout = () => {
    logout();
  };

  return (
    <Navbar expand="lg" className="bg-body-tertiary">
      <Navbar.Brand className="ps-4">Project Suggestions</Navbar.Brand>
      <Navbar.Toggle aria-controls="basic-navbar-nav" />
      <Navbar.Collapse id="basic-navbar-nav">
        <Nav>
          <Link to="/" className="nav-link">
            Home
          </Link>
          {isLoggedIn && <Nav.Link onClick={onClickLogout}>Logout</Nav.Link>}
          {!isLoggedIn && <Nav.Link href={GOOGLE_URL}>Login</Nav.Link>}
        </Nav>
      </Navbar.Collapse>
    </Navbar>
  );
}

export default NavBar;
