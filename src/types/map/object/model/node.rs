use derive_new::new;
use crate::types::maths::*;
use crate::graphics::{ Renderer, Camera, ModelVertexBuffer, Texture, Transform };

#[derive(Clone, Debug, PrototypeElement, new)]
pub struct BoundingBox {
    pub smallest: Vector3<f32>,
    pub biggest: Vector3<f32>,
    pub offset: Vector3<f32>, // move these to function calls?
    pub range: Vector3<f32>, // move these to function calls?
}

impl BoundingBox {
    
    pub fn new_new<'t, T>(vertex_positions: T) -> Self
        where
            T: IntoIterator<Item = Vector3<f32>>,
    {

        let mut smallest: Vector3<f32> = vector3!(999999.0);
        let mut biggest: Vector3<f32> = vector3!(-999999.0);

        for position in vertex_positions {

            smallest.x = smallest.x.min(position.x);
            smallest.y = smallest.y.min(position.y);
            smallest.z = smallest.z.min(position.z);

            biggest.x = biggest.x.max(position.x);
            biggest.y = biggest.y.max(position.y);
            biggest.z = biggest.z.max(position.z);
        }

        let offset = (biggest + smallest).map(|component| component / 2.0);
        let range = (biggest - smallest).map(|component| component / 2.0);

        Self { smallest, biggest, offset, range }
    }

    pub fn center(&self) -> Vector3<f32> {
        self.smallest + self.range
    }
}

#[derive(Clone, PrototypeElement, new)]
pub struct Node {
    pub name: String,
    pub parent_name: Option<String>,
    #[new(default)]
    pub child_nodes: Vec<Node>,
    #[hidden_element]
    pub textures: Vec<Texture>,
    pub transform: Transform,
    #[hidden_element]
    pub vertex_buffer: ModelVertexBuffer,
    pub bounding_box: BoundingBox,
    #[hidden_element]
    pub offset_matrix: Matrix3<f32>,
    pub offset_translation: Vector3<f32>,
    pub position: Vector3<f32>,
    #[hidden_element] // TODO: unhide 
    pub rotation: Vector3<Rad<f32>>,
    pub scale: Vector3<f32>,
}

/*impl Node {

    pub fn render_geometry(&self, renderer: &mut Renderer, camera: &dyn Camera, parent_transform: &Transform) {
        //let combined_transform = *parent_transform + self.transform;
        renderer.render_node(camera, self, parent_transform);
        self.child_nodes.iter().for_each(|node| node.render_geometry(renderer, camera, parent_transform));
    }

    #[cfg(feature = "debug")]
    pub fn render_bounding_box(&self, renderer: &mut Renderer, camera: &dyn Camera, parent_transform: &Transform) {
        let combined_transform = *parent_transform + self.transform + Transform::node_scale(self.bounding_box.range);
        renderer.render_bounding_box(camera, &combined_transform);
        self.child_nodes.iter().for_each(|node| node.render_bounding_box(renderer, camera, &combined_transform));
    }
}*/


#[derive(Clone, PrototypeElement, new)]
pub struct Node2 {
    #[hidden_element]
    pub transform_matrix: Matrix4<f32>,
    #[hidden_element]
    pub vertex_buffer: ModelVertexBuffer,
    #[hidden_element]
    pub textures: Vec<Texture>,
    pub child_nodes: Vec<Node2>,
}

impl Node2 {

    pub fn render_geometry(&self, renderer: &mut Renderer, camera: &dyn Camera, parent_transform: &Transform) {
        renderer.render_node(camera, self, parent_transform);
        self.child_nodes.iter().for_each(|node| node.render_geometry(renderer, camera, parent_transform));
    }
}