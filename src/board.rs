use std::f32::consts::PI;

use bevy::{render::{mesh::{Indices, Mesh}, render_resource::PrimitiveTopology}, prelude::Color};

/// Draw round board with hexagonal filling
pub fn generate_main_board_mesh() -> Mesh {


    return create_cilinder_mesh([0.0, 0.0, 1.0], 1.0, 0.2, 5);
    // return generate_hex_mesh([0.0, 0.0, 0.0], 1.0) 
    
}

fn generate_hex_mesh(center: [f32; 3], radius: f32) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let shape_descr = generate_mesh_shape(center, radius);
    let points: Vec<[f32; 3]> = shape_descr.1.iter().map(|point_idx| shape_descr.0[*point_idx as usize]).collect::<Vec<_>>();
    let normals = calc_normals(&points);
    let colors: Vec<[f32; 4]> = points.iter().map(|point| [point[0], point[1], point[2], 1.0]).collect();

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh
}

fn generate_mesh_shape(center: [f32; 3], radius: f32)  -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut points: Vec<[f32; 3]> = vec![];

    let count_of_sides = 6;

    points.push([center[0], center[1], center[2] + radius]);

    for vertex_idx in 0..count_of_sides{
        let angle = (2. * PI * vertex_idx as f32) / count_of_sides as f32;
        
        points.push([radius * angle.cos(), radius * angle.sin(), center[2] + radius]);
        points.push([radius * angle.cos(), radius * angle.sin(), center[2]]);
    } 

    let mut sides: Vec<u32> = vec![];

    for vertex_idx in 1..count_of_sides + 1 {
        sides.push(0);
        sides.push(2 * vertex_idx - 1);
        sides.push((2 * vertex_idx) % (2 * count_of_sides) + 1);

        sides.push((2 * vertex_idx) % (2 * count_of_sides) + 1);
        sides.push(2 * vertex_idx - 1);
        sides.push((2 * vertex_idx + 1) % (2 * count_of_sides) + 1);

        sides.push(2 * vertex_idx);
        sides.push((2 * vertex_idx + 1) % (2 * count_of_sides) + 1);
        sides.push(2 * vertex_idx - 1);
    }

    return (points, sides)
}

fn create_cilinder_shape(center: [f32; 3],radius: f32, height: f32, magic_number: u32) -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut points: Vec<[f32; 3]> = vec![];

    let count_of_sides = 2 << magic_number;

    points.push([center[0], center[1], center[2] + height / 2.0]);
    points.push([center[0], center[1], center[2] - height / 2.0]);

    for vertex_idx in 0..count_of_sides{
        let angle = (2. * PI * vertex_idx as f32) / count_of_sides as f32;
        
        points.push([radius * angle.cos(), radius * angle.sin(), center[2] + height / 2.0]);
        points.push([radius * angle.cos(), radius * angle.sin(), center[2] - height / 2.0]);
    } 

    let mut sides: Vec<u32> = vec![];
    // points.push([center[0], center[1], center[2] - height / 2.0]);

    for vertex_idx in 1..count_of_sides + 1 {
        sides.push(0);
        sides.push(2 * vertex_idx);
        sides.push((2 * vertex_idx) % (2 * count_of_sides) + 2);

        sides.push((2 * vertex_idx) % (2 * count_of_sides) + 2);
        sides.push(2 * vertex_idx);
        sides.push(2 * vertex_idx + 1);

        sides.push(2 * vertex_idx + 1);
        sides.push((2 * vertex_idx + 1) % (2 * count_of_sides) + 2);
        sides.push((2 * vertex_idx) % (2 * count_of_sides) + 2);

        sides.push(1);
        sides.push(2 * vertex_idx + 1);
        sides.push((2 * vertex_idx + 1) % (2 * count_of_sides) + 2);
    }

    return (points, sides);
}

pub fn calc_normals(points: &Vec<[f32; 3]>) -> Vec<[f32; 3]> {
    
    let mut normals: Vec<[f32; 3]> = vec![];
     
    for triangle_idx in 0..points.len() / 3 {
        let vertex0_idx = triangle_idx * 3;

        let edge1 = [
            points[vertex0_idx + 1][0] - points[vertex0_idx][0],
            points[vertex0_idx + 1][1] - points[vertex0_idx][1],
            points[vertex0_idx + 1][2] - points[vertex0_idx][2]
        ];
    
        let edge2 = [
            points[vertex0_idx + 2][0] - points[vertex0_idx][0],
            points[vertex0_idx + 2][1] - points[vertex0_idx][1],
            points[vertex0_idx + 2][2] - points[vertex0_idx][2]
        ];
    
        // Calculate the cross product of edge1 and edge2 to get the normal vector
        let normal = [
            edge1[1] * edge2[2] - edge1[2] * edge2[1],
            edge1[2] * edge2[0] - edge1[0] * edge2[2],
            edge1[0] * edge2[1] - edge1[1] * edge2[0],
        ];
        
        normals.push(normal);
        normals.push(normal);
        normals.push(normal);
    }
    return normals
}

pub fn create_cilinder_mesh(center: [f32; 3],radius: f32, height: f32, magic_number: u32) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let shape_descr = create_cilinder_shape(center, radius, height, magic_number);
    let points: Vec<[f32; 3]> = shape_descr.1.iter().map(|point_idx| shape_descr.0[*point_idx as usize]).collect::<Vec<_>>();
    let normals = calc_normals(&points);
    let colors: Vec<[f32; 4]> = points.iter().map(|point| [point[0], point[1], point[2], 1.0]).collect();

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh
}
