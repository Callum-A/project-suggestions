import Cookies from 'js-cookie';
import React, { useEffect, useState } from 'react';
import { Button, Form } from 'react-bootstrap';
import { useNavigate, useParams } from 'react-router-dom';

function EditProject() {
  const navigate = useNavigate();
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const { publicId } = useParams();

  useEffect(() => {
    const getProject = async () => {
      if (!publicId) {
        return;
      }
      const resp = await fetch(`/api/v1/project/${publicId}`);
      const data = await resp.json();
      setTitle(data.title);
      setDescription(data.description);
    };
    getProject();
  }, [publicId]);

  const editProject = async (e: React.FormEvent) => {
    e.preventDefault();
    const token = Cookies.get('token');
    if (!token || token === '') {
      return;
    }

    const resp = await fetch(`/api/v1/project/${publicId}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify({
        title,
        description,
        tags_to_add: [],
        tags_to_remove: [],
      }),
    });

    if (resp.status === 200) {
      navigate('/');
    }
  };

  if (!publicId) {
    navigate('/');
    return <h1>Invalid Project</h1>;
  }
  return (
    <div>
      <h1>Create a Project</h1>
      <Form onSubmit={editProject}>
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
          Edit
        </Button>
      </Form>
    </div>
  );
}

export default EditProject;
