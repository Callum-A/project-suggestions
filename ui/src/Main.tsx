import { BrowserRouter, Route, Routes } from 'react-router-dom';
import NavBar from './components/NavBar';
import { Container } from 'react-bootstrap';
import Home from './Home';
import { useEffect } from 'react';
import Cookies from 'js-cookie';
import { useAppState } from './store';

function Main() {
  const logout = useAppState((state) => state.logout);
  const login = useAppState((state) => state.login);
  const setProfile = useAppState((state) => state.setProfile);

  useEffect(() => {
    console.log('Main');
    const getProfile = async (token: string) => {
      const resp = await fetch('/api/v1/user/profile', {
        headers: { Authorization: `Bearer ${token}` },
      });

      if (resp.status !== 200) {
        logout();
        return;
      }

      const data = await resp.json();
      setProfile(data);
    };
    const token = Cookies.get('token');
    if (!token || token === '') {
      logout();
    } else {
      login();
      getProfile(token);
    }
  }, [login, logout, setProfile]);

  return (
    <BrowserRouter>
      <NavBar />
      <Container>
        <Routes>
          <Route path='/' element={<Home />} />
        </Routes>
      </Container>
    </BrowserRouter>
  );
}

export default Main;
