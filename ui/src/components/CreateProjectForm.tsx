import Cookies from 'js-cookie';
import { useState } from 'react';
import { Button, Form } from 'react-bootstrap';
import { useAppState } from '../store';

function CreateProjectForm() {
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [tags, setTags] = useState<string[]>([]);
  const setProjects = useAppState((state) => state.setProjects);

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
  return (
    <div>
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
    </div>
  );
}

export default CreateProjectForm;
