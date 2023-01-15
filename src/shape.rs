use amath::Vec2;

#[derive(Clone, Debug)]
pub struct Body {
    pub transform: Transform,
    pub shape: ConvexPolygon,
}
impl Body {
    pub fn new(transform: Transform, shape: ConvexPolygon) -> Self {
        Self {
            transform,
            shape,
        }
    }
    pub fn compute_vertex_positions(&self) -> ConvexPolygon {
        apply_transform(&self.transform, &self.shape)
    }
    fn hi(self) {
    }
    pub fn is_intersecting(&self, others: Vec<Body>) -> bool {
        let other_refs = Vec::new();
        cvpolyg_intersects_others(
            &self.compute_vertex_positions(),
            other_refs)
    }
}
pub type ConvexPolygon = Vec<Vec2>; // counter-clockwise also convex, for easy use
#[derive(Clone, Debug)]
pub struct Transform {
    pub position: Vec2,
    pub angle: f64,
}
pub fn create_polygon(side_count: usize, side_length: f64) -> ConvexPolygon{
    // compute the starting point
    /*
     * opp/add 
     * so we need to divide by the opposite
     * which in this case happens to be the
     * side_length / 2
     */
    let one_side_angle = (std::f64::consts::PI*2.0) / (side_count as f64);
    let up_unit_vector = Vec2::new(0.0, -1.0);
    let up_left_normal_vector = Vec2::new(-1.0, 0.0);
    let up_dist = (one_side_angle/2.0).tan() * (side_length/2.0);
    let up_vector = up_unit_vector.scale(up_dist);
    let horizontal_offset = up_left_normal_vector.scale(side_length/2.0);
    // start to populate the vertices
    let mut current_point = Vec2::new(up_vector.x+horizontal_offset.x, up_vector.y+horizontal_offset.y);
    let mut current_direction = up_left_normal_vector;
    let mut vertices = Vec::new();
    for _ in 0..side_count {
        vertices.push(current_point);
        current_direction = rotate_vec(current_direction, -one_side_angle);
        current_point = Vec2::new(current_point.x+(current_direction.x * side_length), 
                                  current_point.y + (current_direction.y * side_length));
    }
    vertices
}
fn apply_transform(transform: &Transform, shape: &ConvexPolygon) -> ConvexPolygon {
    let new_vertices = shape.iter().map(|vertex| rotate_vec(*vertex, transform.angle))
        .map(|vertex| Vec2::new(vertex.x+transform.position.x, vertex.y+transform.position.y))
        .collect();
    new_vertices
}
fn rotate_vec(vector: Vec2, angle: f64) -> Vec2 {
    Vec2 {
        x: angle.cos()*vector.x + -angle.sin()*vector.y,
        y: angle.sin()*vector.x + angle.cos()*vector.y
    }
}
pub fn cvpolyg_intersects_others(primary: &ConvexPolygon, others: Vec<&ConvexPolygon>) -> bool {
    // compute the normal of each edge then call it an axis because we can
    for other in others.iter() {
        let mut intersecting = true;
        // attempt to find an axis where the shapes are not overlapping
        for i in 0..primary.len() {
            // get prev and current vertex
            let current_vertex = primary[i];
            let mut prev_vertex = Vec2::new(0.0, 0.0);
            if i > 0 {
                prev_vertex = primary[i-1];
            }
            else {
                prev_vertex = primary[primary.len()-1];
            }
            let edge_direction = Vec2::new(current_vertex.x-prev_vertex.x, current_vertex.y-prev_vertex.y);
            let edge_normal_direction = rotate_vec(edge_direction, std::f64::consts::FRAC_PI_2);
            let axis = edge_normal_direction.scale(1.0/edge_normal_direction.magnitude());

            // projection
            let primary_min = primary.iter().map(|vertex| Vec2::dot(*vertex, axis)).reduce(f64::min).unwrap();
            let primary_max = primary.iter().map(|vertex| Vec2::dot(*vertex, axis)).reduce(f64::max).unwrap();
            let other_min = other.iter().map(|vertex| Vec2::dot(*vertex, axis)).reduce(f64::min).unwrap();
            let other_max = other.iter().map(|vertex| Vec2::dot(*vertex, axis)).reduce(f64::max).unwrap();
            // check for overlap - if no overlap - then intersecting = false
            if primary_max < other_min || other_max < primary_min { // if no overlap
                intersecting = false;
            }
        }
        if intersecting {return true;} // the thing is intersecting
    }
    false
}