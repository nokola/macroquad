use macroquad::*;

fn tree(gl: &mut QuadGl, time: f64, deep: u32, angle: f32, tall: f32) {
    if deep >= 8 {
        return;
    }

    // we can use normal macroquad drawing API here
    draw_rectangle(-0.01 / 2., 0., 0.01, tall, RED);

    // and we can also modify internal macroquad render state with "gl" reference
    gl.push_model_matrix(glam::Mat4::from_translation(glam::vec3(0., tall, 0.)));

    // right leaf
    gl.push_model_matrix(glam::Mat4::from_rotation_z(angle + time.sin() as f32 * 0.1));
    tree(gl, time, deep + 1, angle * 0.7, tall * 0.8);
    gl.pop_model_matrix();

    // left leaf
    gl.push_model_matrix(glam::Mat4::from_rotation_z(
        -angle - time.cos() as f32 * 0.1,
    ));
    tree(gl, time, deep + 1, angle * 0.7, tall * 0.8);
    gl.pop_model_matrix();

    gl.pop_model_matrix();
}

#[macroquad::main("Tree")]
async fn main() {
    set_screen_coordinates(ScreenCoordinates::Fixed(-1., 1., -0.2, 1.4));

    loop {
        clear_background(GREEN);

        draw_circle(0., 0., 0.03, RED);

        tree(unsafe { get_internal_gl() }, get_time(), 0, 1., 0.3);

        next_frame().await
    }
}
