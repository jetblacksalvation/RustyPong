
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
// use rand::Rng; // 0.8.5
fn checkCollision(rect1 :sdl2::rect::Rect , rect2: sdl2::rect::Rect)->bool{
    if rect1.x < rect2.x + rect2.w &&
        rect1.x + rect1.w > rect2.x &&
        rect1.y < rect2.y + rect2.h &&
        rect1.y + rect1.h > rect2.y{
        return true;
    }
    else{
        return false;
    }
}
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();


    let mut paddle = sdl2::rect::Rect::new(40, 350, 30, 100);
    let mut paddle2 = sdl2::rect::Rect::new(740, 350, 30,100);
    let mut ball = sdl2::rect::Rect::new(401,500, 4,4);
    let mut ballvx = 4;
    let mut ballvy = 4;
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255,255,255));

        canvas.fill_rect(paddle);
        canvas.fill_rect(paddle2);
        canvas.fill_rect(ball);
        //clears to white i think.
        for event in event_pump.poll_iter() {
            //game loop match, maybe pass in a lambda to a match statement for each paddle...
            //this would mean that i wouldn't even need to pass in match statement every time, would also need to return ownership of all objects back to main...
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::W),..}=>{
                    println!("w was pressed!");

                    //,.. ignores auto matic vars that are passed
                    paddle.set_y(paddle.y -5);
                },
                Event::KeyDown { keycode: Some(Keycode::S),..}=>{
                    //,.. ignores auto matic vars that are passed
                    println!("S was pressed!");
                    paddle.set_y(paddle.y +5);

                },
                Event::KeyDown { keycode: Some(Keycode::Up),..}=>{
                    println!("w was pressed!");

                    //,.. ignores auto matic vars that are passed
                    paddle2.set_y(paddle2.y -5);
                },
                Event::KeyDown { keycode: Some(Keycode::Down),..}=>{
                    //,.. ignores auto matic vars that are passed
                    println!("S was pressed!");
                    paddle2.set_y(paddle2.y +5);

                }
                _ => {}

            }
        }        // The rest of the game loop goes here...4
        // print!("{} is ballx {} is bally\n", ball.x, ball.y);
        if ball.x+ballvx <0  || ball.x +ballvx >700 || checkCollision( paddle,ball) || checkCollision(paddle2, ball){
            // ballvx = rand::thread_rng().gen_range(0..10);
            ballvx *=-1;

        }
        if ball.y+ballvy < 0 || ball.y +ballvy> 700{
            // ballvy = rand::thread_rng().gen_range(0..10);

            ballvy *=-1;

        }
        ball.set_x(ball.x+ballvx);
        ball.set_y(ball.y + ballvy);

        // canvas.render_flush();
        canvas.present();
        //fllilps the diplay buffer
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));//framerate limit.
    }
}