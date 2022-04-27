use draw::*;
use rand::Rng;

fn main() {
    let mut canvas = Canvas::new(1000,1000);
    const LEFT_POINT: (f32, f32)=  (100.0, 900.0);
    const RIGHT_POINT: (f32, f32) = (900.0,900.0);
    const MID_POINT: (f32, f32) = (500.0, 100.0);

    let draw_left_point = Drawing::new().with_shape(Shape::Rectangle { width: 1, height: 1 })
        .with_xy(LEFT_POINT.0, LEFT_POINT.1)
        .with_style(Style::filled(Color::black()));
        canvas.display_list.add(draw_left_point);

    let draw_right_point = Drawing::new().with_shape(Shape::Rectangle { width: 1, height: 1 })
        .with_xy(RIGHT_POINT.0, RIGHT_POINT.1)
        .with_style(Style::filled(Color::black()));
        canvas.display_list.add(draw_right_point);
    
    let draw_mid_point = Drawing::new().with_shape(Shape::Rectangle { width: 1, height: 1 })
        .with_xy(MID_POINT.0, MID_POINT.1)
        .with_style(Style::filled(Color::black()));
        canvas.display_list.add(draw_mid_point);

    let mut n = 0;
    while n < 500001 {

    let mut rng = rand::thread_rng();
    let random_point: (f32,f32) = (rng.gen_range(0.0..1000.0), rng.gen_range(0.0..1000.0));
    let random_triangle_point = rng.gen_range(1..4);

    let active_point: (f32, f32) = if random_triangle_point == 1 {
       LEFT_POINT 
    } else if random_triangle_point == 2 {
        RIGHT_POINT
    } else {
        MID_POINT 
    };
   
    if is_inside(LEFT_POINT, RIGHT_POINT, MID_POINT, random_point)
    {
    let midpoint: (f32,f32) = ((active_point.0 + random_point.0)/2.0,(active_point.1 + random_point.1)/2.0);

    // let draw_random_point = Drawing::new().with_shape(Shape::Rectangle { width: 1, height: 1 })
    //     .with_xy(random_point.0, random_point.1)
    //     .with_style(Style::filled(Color::black()));
    //     canvas.display_list.add(draw_random_point);

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
             ((first_point.0 *(second_point.1-third_point.1) + 
              second_point.0 * (third_point.1 - first_point.1) + 
              third_point.0 * (first_point.1- second_point.1)
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
