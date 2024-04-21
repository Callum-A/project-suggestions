import React from 'react';
import { Card } from 'react-bootstrap';

const Project: React.FC<{ project: any }> = ({ project }) => {
  return (
    <Card className='mb-2'>
      <Card.Body>
        <Card.Title>{project.title}</Card.Title>
        <Card.Subtitle className='mb-2 text-muted'>
          Created On: {new Date(project.created_at).toLocaleDateString()}
        </Card.Subtitle>
        <Card.Text>{project.description}</Card.Text>
      </Card.Body>
    </Card>
  );
};

export default Project;
