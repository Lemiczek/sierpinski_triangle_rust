use draw::*;
use rand::Rng;

fn main() {
    let mut canvas = Canvas::new(1000,1000);
    const LEFT_POINT: (f32, f32)=  (100.0, 900.0);
    const RIGHT_POINT: (f32, f32) = (900.0,900.0);
    const MID_POINT: (f32, f32) = (500.0, 100.0);

    let mut n = 0;
    let mut next_point: (f32, f32) = (0.0,0.0);
    while n < 500001 {

        let mut rng = rand::thread_rng();
        let random_point: (f32,f32) = (rng.gen_range(0.0..1000.0), rng.gen_range(0.0..1000.0));
        let random_triangle_point = rng.gen_range::<i32,_>(1..4);

        let active_point = match random_triangle_point {
            1 => LEFT_POINT,
            2 => RIGHT_POINT,
            3 => MID_POINT,
            _ => (-1.0, -1.0),
        };
       
        if is_inside(LEFT_POINT, RIGHT_POINT, MID_POINT, random_point)
        {
        let midpoint: (f32,f32) = if n < 1 {
            (
            (active_point.0 + random_point.0)/2.0,
            (active_point.1 + random_point.1)/2.0
            )
        } else {

            (
            (active_point.0 + next_point.0)/2.0,
            (active_point.1 + next_point.1)/2.0
            )
        };
        next_point = midpoint;

        let draw_midpoint = Drawing::new().with_shape(Shape::Rectangle { width: 1, height: 1 })
            .with_xy(midpoint.0, midpoint.1)
            .with_style(Style::filled(Color::black()));
            canvas.display_list.add(draw_midpoint);
            
            n += 1;
        }
    }
render::save(
    &canvas,
    "tests/svg/tri_force.svg",
    SvgRenderer::new(),
    ).expect("Failed to save");

}

fn area(first_point: (f32, f32), second_point: (f32, f32), third_point: (f32,f32)) -> f32 {
             ((first_point.0 *(second_point.1 - third_point.1) + 
              second_point.0 * (third_point.1 - first_point.1) + 
              third_point.0 * (first_point.1 - second_point.1)
              )/2.0
             ).abs()
}
fn is_inside(first_point: (f32, f32), second_point: (f32, f32), third_point: (f32,f32), random_point: (f32, f32)) -> bool {
    let a = area(first_point, second_point, third_point);
    let a1 = area(random_point, second_point, third_point);
    let a2 = area(first_point, random_point, third_point);
    let a3 = area(first_point, second_point, random_point);

    let inside: bool = if a == a1 + a2 + a3 {
        true
    } else {
        false
    };
    inside
}
