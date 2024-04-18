import React, { useEffect, useState } from 'react';
import { useAppState } from './store';
import Cookies from 'js-cookie';
import { Button, Form } from 'react-bootstrap';

function Home() {
  const isLoggedIn = useAppState((state) => state.isLoggedIn);
  const logout = useAppState((state) => state.logout);
  const login = useAppState((state) => state.login);
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [tags, setTags] = useState<string[]>([]);
  const [projects, setProjects] = useState<any[]>([]);
  const getProjects = async () => {
    const resp = await fetch('/api/v1/project');
    const data = await resp.json();
    setProjects(data);
  };

  const createProject = async (e: React.FormEvent) => {
    e.preventDefault();
    console.log(title, description, tags);
    const resp = await fetch('/api/v1/project', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${Cookies.get('token')}`,
      },
      body: JSON.stringify({
        title,
        description,
        tags,
      }),
    });

    // TODO: Error handling
    if (resp.ok) {
      setTitle('');
      setDescription('');
      setTags([]);
      getProjects();
    }
  };

  useEffect(() => {
    const token = Cookies.get('token');
    if (!token || token === '') {
      logout();
    } else {
      login();
    }
    getProjects();
  }, [login, logout]);

  return (
    <div>
      {isLoggedIn && (
        <>
          <h1>Create a Project</h1>
          <Form onSubmit={createProject}>
            <Form.Group className='mb-3' controlId='formBasicEmail'>
              <Form.Label>Title</Form.Label>
              <Form.Control
                type='text'
                placeholder='Enter title'
                value={title}
                onChange={(e) => setTitle(e.target.value)}
              />
            </Form.Group>

            <Form.Group className='mb-3' controlId='formBasicPassword'>
              <Form.Label>Description</Form.Label>
              <Form.Control
                as='textarea'
                placeholder='Description'
                value={description}
                onChange={(e) => setDescription(e.target.value)}
              />
            </Form.Group>
            {/* TODO: Tags */}
            <Button variant='primary' type='submit'>
              Submit
            </Button>
          </Form>
        </>
      )}

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
