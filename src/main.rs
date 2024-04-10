#![feature(iter_intersperse)]
use std::fs;

use dfa::DFA;
use graph::Graph;
use raylib::{misc::get_random_value, prelude::*};
mod dfa;
mod graph;

#[derive(Debug, Clone)]
struct DisplayNodeElement {
    label: String,
    position: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
    size: f32,
    color: Color,
}

#[derive(Debug)]
struct DrawableGraph {
    graph: Graph,
    positions: Vec<DisplayNodeElement>,
}

// Function to rotate a point around another point
fn rotate_point(point: Vector2, pivot: Vector2, angle: f32) -> Vector2 {
    let translated_point = point - pivot;
    let rotated_x = translated_point.x * angle.cos() - translated_point.y * angle.sin();
    let rotated_y = translated_point.x * angle.sin() + translated_point.y * angle.cos();
    Vector2::new(rotated_x, rotated_y) + pivot
}

fn main() {
    let (w, h) = (640, 480);
    let (mut rl, thread) = raylib::init().size(w, h).title("Hello, World").build();

    rl.set_target_fps(60);

    let dfa_code = fs::read_to_string("big.dfa").expect("Failed to read 'test.dfa'");
    let mut graph = DrawableGraph {
        graph: Graph::from(DFA::try_from(dfa_code).unwrap()),
        positions: vec![],
    };

    graph.graph.nodes.iter().for_each(|node| {
        graph.positions.push(DisplayNodeElement {
            position: Vector2 {
                x: f64::from(get_random_value::<i32>(w / 3, 2 * w / 3)) as f32,
                y: f64::from(get_random_value::<i32>(h / 3, 2 * h / 3)) as f32,
            },
            acceleration: Vector2::default(),
            label: node.clone(),
            velocity: Vector2 { x: 0.0, y: 0.0 },
            size: 30.0,
            color: Color::RED,
        })
    });

    // println!("{:#?}", graph);
    // for i in 0..graph.positions.len() {
    //     for j in i + 1..graph.positions.len() {
    //         println!("{i}, {j}");
    //     }
    // }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

        update_graph(&mut graph);
        draw_graph(&mut d, &graph);

        d.draw_circle(w / 2, h / 2, 5.0, Color::YELLOW);
        // break;

        drop(d);
    }
}

fn update_graph(graph: &mut DrawableGraph) {
    for i in 0..graph.positions.len() {
        let center = Vector2::new(320.0, 240.0);
        let x1 = center - graph.positions[i].position;
        graph.positions[i].acceleration = x1.normalized().scale_by(0.01 * x1.length());
    }
    for i in 0..graph.positions.len() {
        for j in i + 1..graph.positions.len() {
            let dir = graph.positions[j].position - graph.positions[i].position;
            if graph.graph.adj_mat.contains_key(&(
                graph.positions[i].label.clone(),
                graph.positions[j].label.clone(),
            )) || graph.graph.adj_mat.contains_key(&(
                graph.positions[j].label.clone(),
                graph.positions[i].label.clone(),
            )) {
                // a = b / d ^ 3
                // graph.positions[i].acceleration +=
                //     dir.normalized().scale_by(dir.length().powi(2) / 10.0);
                // graph.positions[j].acceleration -=
                //     dir.normalized().scale_by(dir.length().powi(2) / 10.0);
                graph.positions[i].acceleration +=
                    dir.normalized().scale_by(0.1 * (dir.length() / 0.2).ln());
                graph.positions[j].acceleration -=
                    dir.normalized().scale_by(0.1 * (dir.length() / 0.2).ln());
                // graph.positions[i].acceleration +=
                //     dir.normalized().scale_by(0.1 * (dir.length() - 200.0));
                // graph.positions[j].acceleration -=
                //     dir.normalized().scale_by(0.1 * (dir.length() - 200.0));
                // graph.positions[i].acceleration += dir.normalized().scale_by(0.1 * dir.length());
                // graph.positions[j].acceleration -= dir.normalized().scale_by(0.1 * dir.length());
            }
            let repulsive_force = dir.scale_by(100.0 / dir.length_sqr());
            graph.positions[i].acceleration -= repulsive_force;
            graph.positions[j].acceleration += repulsive_force;
        }
    }
    for i in 0..graph.positions.len() {
        let node = &mut graph.positions[i];
        node.velocity += node.acceleration.scale_by(0.1);
        node.velocity.scale(0.97);
        node.position += node.velocity.scale_by(1.0);
    }
}

fn draw_graph(mut d: &mut RaylibDrawHandle, graph: &DrawableGraph) {
    graph.positions.iter().enumerate().for_each(|(i, node)| {
        d.draw_circle_v(node.position, node.size, node.color);
        d.draw_text(
            &node.label,
            node.position.x as i32,
            node.position.y as i32,
            15,
            Color::BLACK,
        );
    });

    graph.positions.iter().for_each(|start| {
        graph.positions.iter().for_each(|end| {
            if graph
                .graph
                .adj_mat
                .contains_key(&(start.label.clone(), end.label.clone()))
            {
                draw_edge(&mut d, start, end, 15.0);
            }
        });
    });
}

fn draw_edge(
    d: &mut RaylibDrawHandle,
    start: &DisplayNodeElement,
    end: &DisplayNodeElement,
    arrow_size: f32,
) {
    // Start and end points of the line

    let mut dir = end.position - start.position;
    dir.normalize();
    let perp = Vector2 {
        x: dir.y,
        y: -dir.x,
    };

    let s = start.position + dir * start.size;
    let e = end.position - dir * end.size;
    let t2 = e - dir.scale_by(arrow_size * 0.86 as f32) + perp.scale_by(0.5 * arrow_size);
    let t3 = e - dir.scale_by(arrow_size * 0.86 as f32) - perp.scale_by(0.5 * arrow_size);

    d.draw_line_ex(s, e, 1.0, Color::BLACK);
    d.draw_triangle(e, t2, t3, Color::BLUE);
}
