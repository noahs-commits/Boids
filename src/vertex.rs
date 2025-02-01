use crate::{boid::{self, Boid}, State};



#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub fn get_mesh<'a>(state: &'a State) -> (Vec<Vertex>, Vec<u16>) {
    let mut vert: Vec<Vertex> = Vec::new();

    let mut indexs: Vec<u16>=Vec::new();


    for b in &state.boids{
        add_triangle(b, &mut vert, &mut indexs);
    }

    return (vert,indexs);

}
pub fn add_triangle(pos: &Boid,points: &mut Vec<Vertex>,indexs: &mut Vec<u16>){

    let start_index=points.len() as u16;

    for start_vertex in VERTICES{

        let mut new_vertex=Vertex{position: [
                start_vertex.position[0]*pos.vel[0]-start_vertex.position[1]*pos.vel[1],
                start_vertex.position[1]*pos.vel[0]+start_vertex.position[0]*pos.vel[1],
                0.0
            ], ..*start_vertex
        };
        /*let mut new_vertex=Vertex{
            position: [
                start_vertex.position[0],
                start_vertex.position[1],
                0.0
            ], ..*start_vertex
        };*/

        new_vertex.position[0] += pos.pos[0];
        new_vertex.position[1] += pos.pos[1];
        points.push(new_vertex);
    }

    for i in INDICES{
        indexs.push(i+start_index);
    }
}
const SIZE: f32=0.01;

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [2.0*SIZE, 0.0 , 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [-2.0*SIZE, -SIZE, 0.0],
        color: [ 0.5, 0.0, 0.5],
    },
    Vertex {
        position: [-2.0*SIZE, SIZE, 0.0],
        color: [ 0.5, 0.0, 0.5],
    },
    Vertex {
        position: [-SIZE, 0.0, 0.0],
        color: [0.5, 0.0, 0.5],
    }
];

pub const INDICES: &[u16] = &[
    3, 1, 0,
    0, 2, 3,
];
