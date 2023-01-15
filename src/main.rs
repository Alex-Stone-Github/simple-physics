use amath::Vec2;
mod shape;
use shape::*;

const WIDTH: usize = 60;
const HEIGHT: usize = 40;

fn main() {
    // init
    let mut render_manager = aengine2::RenderManager::new(WIDTH, HEIGHT);

    // shapes
    let body1 = Body::new(Transform {
            position: Vec2::new(15.0, 15.0),
            angle: std::f64::consts::FRAC_PI_2*0.0
        },
        create_polygon(7, 12.0)
    );
    let body2 = Body::new(Transform {
            position: Vec2::new(40.0, 20.0),
            angle: 0.0
        },
        create_polygon(4, 5.0)
    );

    // drawing
    render_manager.fill_background('.');
    // shape 1
    draw_shape(&mut render_manager, &body1.compute_vertex_positions());
    // shape 2
    draw_shape(&mut render_manager, &body2.compute_vertex_positions());
    render_manager.present();
    // info
    println!("{}", cvpolyg_intersects_others(&body1.compute_vertex_positions(), vec![&body2.compute_vertex_positions()]));
}


fn draw_shape(rm: &mut aengine2::RenderManager, shape: &ConvexPolygon) {
    for i in 1..shape.len() {
        let vertex = shape[i];
        let pvertex = shape[i-1];
        rm.draw_line(pvertex.x as usize, pvertex.y as usize, 
                                 vertex.x as usize, vertex.y as usize, '#');
    }
    rm.draw_line(shape[shape.len()-1].x as usize, shape[shape.len()-1].y as usize, 
                             shape[0].x as usize, shape[0].y as usize, '#');
}
