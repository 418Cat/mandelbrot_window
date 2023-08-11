extern crate sdl2;

use mand::mand_colors;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::Canvas;

use colorsys;
use serde::{Serialize, Deserialize};
use std::env;

mod mand;

#[derive(Serialize, Deserialize)]
struct Conf
{
    res: [u32; 2],
    img_output_res: [u32; 2],
    start_rect: [f64; 2],
    rect_size: [f64; 2],
    limit: u32,
    zoom: f64,
    color_fn : mand_colors::ColorFn
}

impl std::default::Default for Conf {
    fn default() -> Self
    {
        Self
        {
            res: [500, 500],
            img_output_res: [1000, 1000],
            start_rect: [-1.5, -1.],
            rect_size: [2., 2.],
            limit: 100,
            zoom: 5.,
            color_fn: mand_colors::ColorFn::Colors6
        }
    }
}

pub fn main() {

    let (store_dir, output_dir) = get_dirs();

    let config_file: Conf = confy::load_path::<Conf>(format!("{store_dir}/config")).unwrap();

    let res: [u32; 2] = config_file.res;
    let mut start_rect: [f64; 2] = config_file.start_rect;
    let mut rect_size: [f64; 2] = config_file.rect_size;
    let mut limit: u32 = config_file.limit;
    let zoom: f64 = config_file.zoom;

    let color_fn = mand::mand_colors::get_fn_from_enum(config_file.color_fn);
    
    let (mut canvas, mut event_pump) = init_canvas(res);

    draw_mand(start_rect, rect_size, res, &mut canvas, Some(limit), color_fn);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseWheel { timestamp:_ , window_id:_ , which:_ , x:_ , y, direction:_  } =>
                {
                    if y != 0
                    {
                        zoom_fn(&mut start_rect, &mut rect_size, zoom * y as f64);
                        draw_mand(start_rect, rect_size, res, &mut canvas, Some(limit), color_fn);
                    }
                },
                Event::MouseButtonDown { timestamp: _, window_id: _, which: _, mouse_btn: _, clicks: _, x, y } =>
                {
                    let mouse_in_rect = mouse_pos_to_rect_pos([x as u32, y as u32], res, start_rect, rect_size);
                    
                    start_rect = 
                    [
                        mouse_in_rect[0] - rect_size[0]/2.,
                        mouse_in_rect[1] - rect_size[1]/2.
                    ];
                    draw_mand(start_rect, rect_size, res, &mut canvas, Some(limit), color_fn);
                }
                Event::KeyDown { timestamp: _, window_id: _, keycode, scancode: _, keymod: _, repeat: _ } =>
                {
                    let keycode = keycode.unwrap();
                    match keycode
                    {
                        Keycode::Right =>
                        {
                            limit+= (limit as f64 / 4.).ceil() as u32;
                            draw_mand(start_rect, rect_size, res, &mut canvas, Some(limit), color_fn);
                        },
                        Keycode::Left =>
                        {
                            limit-= (limit as f64 / 4.) as u32;
                            draw_mand(start_rect, rect_size, res, &mut canvas, Some(limit), color_fn);
                        },
                        Keycode::Up =>
                        {
                            zoom_fn(&mut start_rect, &mut rect_size, -zoom);
                            draw_mand(start_rect, rect_size, res, &mut canvas, Some(limit), color_fn);
                        },
                        Keycode::Down =>
                        {
                            zoom_fn(&mut start_rect, &mut rect_size, zoom);
                            draw_mand(start_rect, rect_size, res, &mut canvas, Some(limit), color_fn);
                        },
                        Keycode::Return =>
                        {
                            save_img_buff(start_rect, rect_size, config_file.img_output_res, Some(limit), config_file.color_fn, output_dir.clone());
                        },
                        Keycode::KpEnter =>
                        {
                            save_img_buff(start_rect, rect_size, config_file.img_output_res, Some(limit), config_file.color_fn, output_dir.clone());
                        },
                        Keycode::Space =>
                        {
                            while rect_size[0] < 2.0
                            {
                                zoom_fn(&mut start_rect, &mut rect_size, zoom);
                                save_img_buff(start_rect, rect_size, config_file.img_output_res, Some(limit), config_file.color_fn, output_dir.clone());
                            }
                        },
                        _ => {}
                    }
                    
                }
                _ => {}
            }
        }
    }
}

fn draw_mand(start_rect: [f64; 2], rect_size: [f64; 2], res: [u32; 2], canvas :&mut Canvas<sdl2::video::Window>, limit: Option<u32>, color_fn: fn(u32, u32) -> colorsys::Rgb)
{
    let [x_res_factor, y_res_factor]: [f64; 2] = [rect_size[0]/res[0] as f64, rect_size[1]/res[1] as f64];

    let limit: u32 = limit.unwrap_or(100);

    for x in 0..res[0]
    {
        for y in 0..res[1]
        {
            let n: u32 = mand::mand::get_mand_point([start_rect[0] + x_res_factor * x as f64, start_rect[1] + y_res_factor * y as f64], Some(limit));
            let color = color_fn(n, limit);
            canvas.set_draw_color(sdl2::pixels::Color::RGB(color.red() as u8, color.green() as u8, color.blue() as u8));
            canvas.draw_point(Point::new(x as i32, y as i32)).expect("Could not draw point");
        }
        print!("\r[{}>{}] {:.2}%. iter_nb:{}       ","=".repeat((x as f32 / res[0] as f32 * 50.) as usize), " ".repeat(49 - (x as f32 / res[0] as f32 * 50.) as usize), x as f32 / res[0] as f32 * 100., limit);
    }

    let title = format!("({:.10}, {:.10});({:.10}, {:.10})", start_rect[0], start_rect[1], rect_size[0], rect_size[1]);
    canvas.window_mut().set_title(&title).expect("Could not set title");

    canvas.present();
}

fn mouse_pos_to_rect_pos(mouse_coords: [u32; 2], res: [u32; 2], start_rect: [f64; 2], rect_size: [f64; 2]) -> [f64; 2]
{
    [start_rect[0] + (mouse_coords[0] as f64 / res[0] as f64) * rect_size[0],
    start_rect[1] + (mouse_coords[1] as f64 / res[1] as f64) * rect_size[1]]
}

fn init_canvas(res: [u32; 2]) -> (Canvas<sdl2::video::Window>, EventPump)
{
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("mandelbrot", res[0], res[1])
        .position_centered()
        .build()
        .unwrap();

    (window.into_canvas().build().unwrap(), sdl_context.event_pump().unwrap())
}

fn save_img_buff(start_rect: [f64; 2], rect_size: [f64; 2], img_output_res: [u32; 2], limit: Option<u32>, color_fn: mand_colors::ColorFn, output_dir: String)
{
    let file_name = format!("coords:[{},{}];size:[{},{}];max:{}.png", start_rect[0], start_rect[1], rect_size[0], rect_size[1], limit.unwrap_or(100));
    //let file_name = format!("{}/{}.png", output_dir.clone(), SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());

    match image::save_buffer(format!("{}/{}", output_dir, file_name), &mand::mand::get_mand_buff_img(start_rect, rect_size, img_output_res, limit, color_fn), img_output_res[0], img_output_res[1], image::ColorType::Rgb8)
    {
        Err(e) => println!("{e}"),
        Ok(_) => {},
    }
}

fn zoom_fn(start_rect: &mut [f64; 2], rect_size: &mut [f64; 2], zoom: f64)
{
    *start_rect = [
        start_rect[0] - (rect_size[0]/zoom),
        start_rect[1] - (rect_size[1]/zoom)];

    *rect_size = [
        rect_size[0] + (rect_size[0]/zoom) * 2.,
        rect_size[1] + (rect_size[1]/zoom) * 2.];
}

fn get_dirs() -> (String, String) //store_dir, output_dir
{
    let mut store_dir = String::from(".");

    match env::current_exe() {
        Ok(exe_path) =>
        {
            store_dir = format!("{}", exe_path.parent().expect("Could not get the parent dir of the current exe").display());
        }
        Err(e) => println!("failed to get current exe path: {e}\nWill use the current dir instead"),
    };

    let output_dir = format!("{}/output", store_dir);
    println!("{store_dir}");

    match std::fs::read_dir(output_dir.clone())
    {
        Err(e) => 
        {
            if e.raw_os_error().unwrap() == 2
            {
                let dirbuild = std::fs::DirBuilder::new();
                match dirbuild.create(output_dir.clone())
                {
                    Err(e) =>
                    {
                        panic!("Couldn't create directory '{}'. Try creating it manually\n{}", output_dir.clone(), e);
                    }
                    Ok(_) =>
                    {
                        println!("Created '{}' dir", output_dir.clone())
                    }
                }
            }
            else
            {
                println!("{e}");
                panic!("There was a problem checking if the '{}' dir exists. Make sure the program has the right permissions", output_dir.clone())
            }
        }
        Ok(_) => {}
    }

    (store_dir, output_dir)
}