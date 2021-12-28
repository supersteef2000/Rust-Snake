use std::path::Path;
use std::time::Duration;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let height = 1080;
    let unit = height / 18;
    let width = 1920;
    let window = video_subsystem.window("Window", width as u32, height as u32).fullscreen().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let snake = sdl2::surface::Surface::load_bmp(Path::new("assets/snake.bmp")).unwrap();
    let wall = sdl2::surface::Surface::load_bmp(Path::new("assets/wall.bmp")).unwrap();
    let apple = sdl2::surface::Surface::load_bmp(Path::new("assets/apple.bmp")).unwrap();
    let numbers = sdl2::surface::Surface::load_bmp(Path::new("assets/numbers.bmp")).unwrap();
    let snake_texture = texture_creator
        .create_texture_from_surface(&snake)
        .map_err(|e| e.to_string()).unwrap();
    let wall_texture = texture_creator
        .create_texture_from_surface(&wall)
        .map_err(|e| e.to_string()).unwrap();
    let apple_texture = texture_creator
        .create_texture_from_surface(&apple)
        .map_err(|e| e.to_string()).unwrap();
    let numbers_texture = texture_creator
        .create_texture_from_surface(&numbers)
        .map_err(|e| e.to_string()).unwrap();
    let snake_src = Rect::new(0, 0, 16, 16);
    let mut snake_dst = Rect::new(0, 0, unit as u32, unit as u32);
    let wall_src = Rect::new(0, 0, 16, 16);
    let mut wall_dst = Rect::new(0, 0, unit as u32, unit as u32);
    let apple_src = Rect::new(0, 0, 16, 16);
    let mut apple_dst = Rect::new(0, 0, unit as u32, unit as u32);
    let mut first_digit_src = Rect::new(0, 0, 3, 5);
    let mut second_digit_src = Rect::new(0, 0, 3, 5);
    let mut third_digit_src = Rect::new(0, 0, 3, 5);
    let first_digit_dst = Rect::new(5 * unit, 4 * unit, (6 * unit) as u32, (10 * unit) as u32);
    let second_digit_dst = Rect::new(13 * unit, 4 * unit, (6 * unit) as u32, (10 * unit) as u32);
    let third_digit_dst = Rect::new(21 * unit, 4 * unit, (6 * unit) as u32, (10 * unit) as u32);
    let mut snake_x = Vec::new();
    snake_x.insert(0, snake_dst.x());
    let mut snake_y = Vec::new();
    snake_y.insert(0, snake_dst.y());
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut current_x = width / 2 - width / 2 % unit;
    let mut current_y = height / 2 - height / 2 % unit;
    let mut key = rand::thread_rng().gen_range(1..5);
    let mut new_apple_created = false;
    let difficulty = 5.0;
    let mut apple_x = 0;
    let mut apple_y = 0;
    let mut score = 0;
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } |
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => if snake_x.len() == 1 || snake_y[0] <= snake_y[1] { key = 1 },
                Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } |
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => if snake_x.len() == 1 || snake_x[0] <= snake_x[1] { key = 2 },
                Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } |
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => if snake_x.len() == 1 || snake_y[0] >= snake_y[1] { key = 3 },
                Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } |
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => if snake_x.len() == 1 || snake_x[0] >= snake_x[1] { key = 4 },
                _ => ()
            }
        }

        match key {
            1 => current_y -= unit,
            2 => current_x -= unit,
            3 => current_y += unit,
            4 => current_x += unit,
            _ => ()
        }

        snake_x.insert(0, current_x);
        snake_y.insert(0, current_y);
        snake_x.remove(snake_x.len() - 1);
        snake_y.remove(snake_y.len() - 1);

        while wall_dst.x() < width - unit {
            wall_dst.set_y(0);
            wall_dst.set_x(wall_dst.x() + unit);
            canvas.copy_ex(&wall_texture, wall_src, wall_dst, 0.0, None, false, false).unwrap();
        }
        while wall_dst.y() < height - unit {
            wall_dst.set_x(width - unit);
            wall_dst.set_y(wall_dst.y() + unit);
            canvas.copy_ex(&wall_texture, wall_src, wall_dst, 0.0, None, false, false).unwrap();
        }
        while wall_dst.x() > 0 {
            wall_dst.set_y(height - unit);
            wall_dst.set_x(wall_dst.x() - unit);
            canvas.copy_ex(&wall_texture, wall_src, wall_dst, 0.0, None, false, false).unwrap();
        }
        while wall_dst.y() > 0 {
            wall_dst.set_x(0);
            wall_dst.set_y(wall_dst.y() - unit);
            canvas.copy_ex(&wall_texture, wall_src, wall_dst, 0.0, None, false, false).unwrap();
        }

        if snake_x[0] == apple_dst.x() && snake_y[0] == apple_dst.y() {
            snake_x.insert(snake_x.len(), snake_x[snake_x.len() - 1]);
            snake_y.insert(snake_y.len(), snake_y[snake_y.len() - 1]);
            score += 1;
            new_apple_created = false
        }

        if new_apple_created == false {
            apple_x = rand::thread_rng().gen_range(1..31);
            apple_y = rand::thread_rng().gen_range(1..17);
            apple_dst.set_x(apple_x * unit);
            apple_dst.set_y(apple_y * unit);
            new_apple_created = true;
        }


        apple_dst.set_x(apple_x * unit);
        apple_dst.set_y(apple_y * unit);

        if snake_x.len() > 2 {
            for snake_part in 1..snake_x.len() {
                if snake_x[0] == snake_x[snake_part] && snake_y[0] == snake_y[snake_part] {
                    break 'running
                }
            }
        }

        if snake_x[0] < unit || snake_x[0] > width - 2 * unit || snake_y[0] < unit || snake_y[0] > height - 2 * unit {
            break 'running;
        }

        first_digit_src.set_x((score - score % 100) * 3 / 100);
        second_digit_src.set_x((score % 100 - score % 100 % 10) * 3 / 10);
        third_digit_src.set_x(score % 10 * 3);

        canvas.copy_ex(&numbers_texture, first_digit_src, first_digit_dst, 0.0, None, false, false).unwrap();
        canvas.copy_ex(&numbers_texture, second_digit_src, second_digit_dst, 0.0, None, false, false).unwrap();
        canvas.copy_ex(&numbers_texture, third_digit_src, third_digit_dst, 0.0, None, false, false).unwrap();
        canvas.copy_ex(&apple_texture, apple_src, apple_dst, 0.0, None, false, false).unwrap();
        for snake_part in snake_x.clone().iter().zip(snake_y.clone().iter()) {
            snake_dst.set_x(*snake_part.0);
            snake_dst.set_y(*snake_part.1);
            canvas.copy_ex(&snake_texture, snake_src, snake_dst, 0.0, None, false, false).unwrap();
        }
        canvas.present();

        let nanos: f32 = 1000000000.0 / difficulty;
        std::thread::sleep(Duration::new(0, nanos as u32));
        //difficulty += 0.01;
    }
    println!("Final score: {}", score);
    let mut end_dst = snake_dst;
    end_dst.set_x(snake_x[0]);
    end_dst.set_y(snake_y[0]);
    let mut blinking_src;
    for _i in 1..10 {
        blinking_src = apple_src;
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        while wall_dst.x() < width - unit {
            wall_dst.set_y(0);
            wall_dst.set_x(wall_dst.x() + unit);
            canvas.copy_ex(&wall_texture, wall_src, wall_dst, 0.0, None, false, false).unwrap();
        }
        while wall_dst.y() < height - unit {
            wall_dst.set_x(width - unit);
            wall_dst.set_y(wall_dst.y() + unit);
            canvas.copy_ex(&wall_texture, wall_src, wall_dst, 0.0, None, false, false).unwrap();
        }
        while wall_dst.x() > 0 {
            wall_dst.set_y(height - unit);
            wall_dst.set_x(wall_dst.x() - unit);
            canvas.copy_ex(&wall_texture, wall_src, wall_dst, 0.0, None, false, false).unwrap();
        }
        while wall_dst.y() > 0 {
            wall_dst.set_x(0);
            wall_dst.set_y(wall_dst.y() - unit);
            canvas.copy_ex(&wall_texture, wall_src, wall_dst, 0.0, None, false, false).unwrap();
        }
        canvas.copy_ex(&numbers_texture, first_digit_src, first_digit_dst, 0.0, None, false, false).unwrap();
        canvas.copy_ex(&numbers_texture, second_digit_src, second_digit_dst, 0.0, None, false, false).unwrap();
        canvas.copy_ex(&numbers_texture, third_digit_src, third_digit_dst, 0.0, None, false, false).unwrap();
        canvas.copy_ex(&apple_texture, apple_src, apple_dst, 0.0, None, false, false).unwrap();
        for snake_part in snake_x.clone().iter().zip(snake_y.clone().iter()) {
            snake_dst.set_x(*snake_part.0);
            snake_dst.set_y(*snake_part.1);
            canvas.copy_ex(&snake_texture, snake_src, snake_dst, 0.0, None, false, false).unwrap();
        }
        canvas.copy_ex(&apple_texture, blinking_src, end_dst, 0.0, None, false, false).unwrap();
        canvas.present();
        std::thread::sleep(Duration::new(0, 200000000));
        blinking_src = snake_src;
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        while wall_dst.x() < width - unit {
            wall_dst.set_y(0);
            wall_dst.set_x(wall_dst.x() + unit);
            canvas.copy_ex(&wall_texture, wall_src, wall_dst, 0.0, None, false, false).unwrap();
        }
        while wall_dst.y() < height - unit {
            wall_dst.set_x(width - unit);
            wall_dst.set_y(wall_dst.y() + unit);
            canvas.copy_ex(&wall_texture, wall_src, wall_dst, 0.0, None, false, false).unwrap();
        }
        while wall_dst.x() > 0 {
            wall_dst.set_y(height - unit);
            wall_dst.set_x(wall_dst.x() - unit);
            canvas.copy_ex(&wall_texture, wall_src, wall_dst, 0.0, None, false, false).unwrap();
        }
        while wall_dst.y() > 0 {
            wall_dst.set_x(0);
            wall_dst.set_y(wall_dst.y() - unit);
            canvas.copy_ex(&wall_texture, wall_src, wall_dst, 0.0, None, false, false).unwrap();
        }
        canvas.copy_ex(&numbers_texture, first_digit_src, first_digit_dst, 0.0, None, false, false).unwrap();
        canvas.copy_ex(&numbers_texture, second_digit_src, second_digit_dst, 0.0, None, false, false).unwrap();
        canvas.copy_ex(&numbers_texture, third_digit_src, third_digit_dst, 0.0, None, false, false).unwrap();
        canvas.copy_ex(&apple_texture, apple_src, apple_dst, 0.0, None, false, false).unwrap();
        for snake_part in snake_x.clone().iter().zip(snake_y.clone().iter()) {
            snake_dst.set_x(*snake_part.0);
            snake_dst.set_y(*snake_part.1);
            canvas.copy_ex(&snake_texture, snake_src, snake_dst, 0.0, None, false, false).unwrap();
        }
        canvas.copy_ex(&snake_texture, blinking_src, end_dst, 0.0, None, false, false).unwrap();
        canvas.present();
        std::thread::sleep(Duration::new(0, 200000000));
    }
}
