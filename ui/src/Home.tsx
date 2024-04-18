import React, { useEffect } from 'react';
import { useAppState } from './store';
import Cookies from 'js-cookie';
import CreateProjectForm from './components/CreateProjectForm';

function Home() {
  const isLoggedIn = useAppState((state) => state.isLoggedIn);
  const logout = useAppState((state) => state.logout);
  const login = useAppState((state) => state.login);
  const projects = useAppState((state) => state.projects);
  const setProjects = useAppState((state) => state.setProjects);

  useEffect(() => {
    const token = Cookies.get('token');
    if (!token || token === '') {
      logout();
    } else {
      login();
    }
  }, [login, logout]);

  useEffect(() => {
    const getProjects = async () => {
      const resp = await fetch('/api/v1/project');
      const data = await resp.json();
      setProjects(data);
    };
    getProjects();
  }, [setProjects]);

  return (
    <div>
      {isLoggedIn && <CreateProjectForm />}

      <h1>Projects</h1>
      <div>
        {projects.map((project) => (
          <div key={project.id}>
            <h3>{project.title}</h3>
            <p>{project.description}</p>
          </div>
        ))}
      </div>
    </div>
  );
}

export default Home;
