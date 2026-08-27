use std::ops::Index;

use glam::Vec2;

fn cycle_index(i: i32, len: usize) -> usize {
    let len = len as i32;
    (((i % len) + len) % len) as usize
}

fn cycle<T>(v: &Vec<T>, idx: i32) -> &T {
    v.index(cycle_index(idx, v.len()))
}

#[derive(PartialEq)]
struct Node {
    idx: u32,
    pos: Vec2,
}

fn area(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    (b.y - a.y) * (c.x - b.x) - (b.x - a.x) * (c.y - b.y)
}

fn is_in_triangle(a: Vec2, b: Vec2, c: Vec2, p: Vec2) -> bool {
    (c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y) >= 0.0
        && (a.x - p.x) * (b.y - p.y) - (b.x - p.x) * (a.y - p.y) >= 0.0
        && (b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y) >= 0.0
}

fn is_ear(nodes: &Vec<Node>, i: i32) -> bool {
    let (a, b, c) = (cycle(nodes, i - 1), cycle(nodes, i), cycle(nodes, i + 1));

    if area(a.pos, b.pos, c.pos) >= 0.0 {
        return false;
    }

    for (node, i) in nodes.iter().zip(0..(nodes.len() as i32)) {
        if node == a || node == b || node == c {
            continue;
        }

        if is_in_triangle(a.pos, b.pos, c.pos, node.pos)
            && area(cycle(nodes, i - 1).pos, node.pos, cycle(nodes, i + 1).pos) >= 0.0
        {
            return false;
        }
    }

    true
}

pub(crate) fn triangulate(points: &[Vec2], idx_offset: u32) -> Option<Vec<u32>> {
    let mut res = vec![];

    let mut nodes: Vec<Node> = points
        .iter()
        .enumerate()
        .map(|(idx, pos)| Node {
            idx: idx as u32 + idx_offset,
            pos: *pos,
        })
        .collect();

    let mut cn = 0;
    let mut failed = 0;
    while nodes.len() > 2 {
        if is_ear(&nodes, cn) {
            res.extend_from_slice(&[
                cycle(&nodes, cn - 1).idx,
                cycle(&nodes, cn).idx,
                cycle(&nodes, cn + 1).idx,
            ]);

            nodes.remove(cycle_index(cn, nodes.len()));
        } else {
            cn += 1;
            failed += 1;
            if failed >= nodes.len() {
                return None;
            }
        }
    }

    Some(res)
}
