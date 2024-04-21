import React from 'react';
import { Card } from 'react-bootstrap';
import { useAppState } from '../store';
import Cookies from 'js-cookie';
import { useNavigate } from 'react-router-dom';

const Project: React.FC<{ project: any }> = ({ project }) => {
  const profile = useAppState((state) => state.profile);
  const setProjects = useAppState((state) => state.setProjects);
  const projects = useAppState((state) => state.projects);
  const navigate = useNavigate();

  const onClickEdit = (publicId: string) => {
    console.log('Edit', publicId);
    navigate(`/edit/${publicId}`);
  };

  const onClickDelete = (publicId: string) => {
    console.log('Delete', publicId);
    const token = Cookies.get('token');
    if (!token || token === '') {
      return;
    }

    fetch(`/api/v1/project/${publicId}`, {
      method: 'DELETE',
      headers: {
        Authorization: `Bearer ${token}`,
      },
    }).then((resp) => {
      if (resp.status === 200) {
        const newProjects = projects.filter(
          (project: any) => project.public_id !== publicId
        );
        setProjects(newProjects);
      }
    });
  };

  return (
    <Card className='mb-2'>
      <Card.Body>
        <Card.Title>{project.title}</Card.Title>
        <Card.Subtitle className='mb-2 text-muted'>
          Created On: {new Date(project.created_at).toLocaleDateString()}
        </Card.Subtitle>
        <Card.Text>{project.description}</Card.Text>
        {profile?.id === project.user_id && (
          <>
            <Card.Link onClick={(e) => onClickDelete(project.public_id)}>
              Delete
            </Card.Link>
            <Card.Link onClick={(e) => onClickEdit(project.public_id)}>
              Edit
            </Card.Link>
          </>
        )}
      </Card.Body>
    </Card>
  );
};

export default Project;
