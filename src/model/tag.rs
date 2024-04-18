#[derive(Debug)]
pub struct Tag {
    pub id: i32,
    pub public_id: String,
    pub name: String,
}

#[derive(Debug)]
pub struct TagToProject {
    pub tag_id: i32,
    pub project_id: i32,
}
