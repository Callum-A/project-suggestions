import React, { useEffect } from 'react';
import { useAppState } from './store';
import Cookies from 'js-cookie';
import CreateProjectForm from './components/CreateProjectForm';
import Project from './components/Project';
import { Pagination } from 'react-bootstrap';

const PER_PAGE = 10;

function Home() {
  const [page, setPage] = React.useState(1);
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
      const resp = await fetch(
        '/api/v1/project?page=' + page + '&limit=' + PER_PAGE
      );
      const data = await resp.json();
      setProjects(data);
    };
    getProjects();
  }, [setProjects, page]);

  return (
    <div>
      {isLoggedIn && <CreateProjectForm />}

      <h1>Projects</h1>
      <div>
        <div className='d-flex justify-content-center'>
          <Pagination>
            <Pagination.Prev
              onClick={() => setPage(page - 1)}
              disabled={page === 1}
            />
            <Pagination.Item disabled>{page}</Pagination.Item>
            <Pagination.Next
              onClick={() => setPage(page + 1)}
              disabled={projects.length < PER_PAGE}
            />
          </Pagination>
        </div>
        <div>
          {projects.map((project) => (
            <Project key={project.id} project={project} />
          ))}
        </div>
        <div className='d-flex justify-content-center'>
          <Pagination>
            <Pagination.Prev
              onClick={() => setPage(page - 1)}
              disabled={page === 1}
            />
            <Pagination.Item disabled>{page}</Pagination.Item>
            <Pagination.Next
              onClick={() => setPage(page + 1)}
              disabled={projects.length < PER_PAGE}
            />
          </Pagination>
        </div>
      </div>
    </div>
  );
}

export default Home;
