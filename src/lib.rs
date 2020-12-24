use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{self};
use rand::prelude::*;

struct Player {
    hp: u8,
    tp: u8,
    pos: Point,
}
struct Ray {
    pos: Point,
    angle: f64,
}
struct Point {
    x: f64,
    y: f64,
}
struct Wall {
    start: f64,
    end: f64,
    width: f64
}
trait ToJson {
    fn transform_to_json(&self) -> String;
}
impl ToJson for Player {
    fn transform_to_json(&self) -> String {
        let ret = String::from(format!(
            "\"hp\": {0}, \"tp\": {1}",
            self.hp, self.tp
        ));
        ret
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let canvas = document
        .get_element_by_id("rust-canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let _context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    Ok(())
}

#[wasm_bindgen]
pub fn rust_draw_bitmap(
    context: &web_sys::CanvasRenderingContext2d,
    image: &web_sys::ImageBitmap,
    dest_x: f64,
    dest_y: f64,
) -> Result<(), JsValue> {
    let canvas = context
        .canvas()
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
    context.draw_image_with_image_bitmap(image, dest_x, dest_y)?;
    Ok(())
}

#[wasm_bindgen]
pub fn raycaster(
    context: &web_sys::CanvasRenderingContext2d,
    map: &str,
    //image: &web_sys::ImageBitmap,
    angle: f64,
    player_x: i32,
    player_y: i32
) -> Result<(), JsValue> {
    context.clear_rect(0.0, 0.0, 800.0, 600.0);
    let mut map_as_i32 = vec![vec![0; 8]; 8];
    let mut index_in_row: usize = 0;
    let mut index_in_col: usize = 0;

    
    for (_index, c) in map.chars().enumerate() {
        match c {
            '1' => map_as_i32[index_in_row][index_in_col] = 1,
            '0' => map_as_i32[index_in_row][index_in_col] = 0,
            _ => map_as_i32[index_in_row][index_in_col] = -1,
        }
        if index_in_col == 7 {
            index_in_row += 1;
            index_in_col = 0;
        } else {
            index_in_col += 1;
        }
    }

    for i in 0..800 {
        let fov: f64 = 32.0; // field of view in degrees
        let mut wall_hit = false;
        let mut step = Point { x: 0.0, y: 0.0 };
        let mut wall_dist = Point { x: 0.0, y: 0.0 };
        let ray_angle_rad = (angle as f64 + (-(fov / 2.0) + ( i as f64 * ((fov / 2.0) / 800.0) ))) * (3.14 / 180.0);
        let mut ray = Ray {
            angle: angle * (3.14 / 180.0),
            pos: Point { x: player_x as f64 + ray_angle_rad.cos(), y: player_y as f64 + ray_angle_rad.sin()}
        };

        if ray.pos.x < 0.0 {
            ray.pos.x = 1.5;
        }
        if ray.pos.y < 0.0 {
            ray.pos.y = 1.5;
        }
        step.x = ray.angle.cos();
        step.y = ray.angle.sin();

        while !wall_hit {
            if map_as_i32[ray.pos.y.round() as usize][ray.pos.x.round() as usize] > 0 {
                wall_hit = true;
            } else {
                ray.pos.x += step.x / 50.0;
                ray.pos.y += step.y / 50.0;
                wall_dist.x += step.x;
                wall_dist.y += step.y;
            }
        }
        let wall_dist_hyp = ((wall_dist.x * wall_dist.x) + (wall_dist.y * wall_dist.y)).sqrt();

        let draw_start = 30.0 + wall_dist_hyp;
        let draw_end = 600.0 - draw_start;

        let wall = Wall{ start: draw_start, end: draw_end - draw_start, width: 1.0 };
        context.set_fill_style(&JsValue::from_str(format!("rgb({0}, {0}, {0})", 
            (255.0 / (2.0 * wall_dist_hyp/ 100.0) ) ).as_str()));
        context.fill_rect(i as f64, wall.start, wall.width, wall.end );
    }

    Ok(())
}

#[wasm_bindgen]
pub fn raycast_from_top(
    context: &web_sys::CanvasRenderingContext2d,
    map: &str,
    angle: f64,
    player_x: f64,
    player_y: f64
) -> Result<(), JsValue> {
    context.clear_rect(0.0, 0.0, 200.0, 200.0);
    let mut map_as_i32 = vec![vec![0; 8]; 8];
    let mut index_in_row: usize = 0;
    let mut index_in_col: usize = 0;
    
    // let angle = angle * (3.14 / 180.0);
    context.set_fill_style(&JsValue::from_str("grey"));
    for (_index, c) in map.chars().enumerate() {
        match c {
            '1' => map_as_i32[index_in_row][index_in_col] = 1,
            '0' => map_as_i32[index_in_row][index_in_col] = 0,
            _ => map_as_i32[index_in_row][index_in_col] = -1,
        }
        if index_in_col == 7 {
            index_in_row += 1;
            index_in_col = 0;
        } else {
            index_in_col += 1;
        }
    }

    for (index_row, row) in map_as_i32.iter().enumerate() {
        for (index_col, col) in row.iter().enumerate() {
            if col > &0 {
                context.fill_rect(25.0 * index_col as f64, 25.0 * index_row as f64, 25.0, 25.0);
            }
        }
    }

    //Begin to draw
    context.begin_path();
    context.set_stroke_style(&JsValue::from_str("blue"));
        let mut wall_hit = false;
        let mut step = Point { x: 1.0, y: 0.0 };
        let mut wall_dist = Point { x: 0.0, y: 0.0 };
        //let ray_angle_rad = (angle as f64 + (-32.0 + i as f64)) * (3.14 / 180.0);
        let ray_angle_rad = angle * (3.14 / 180.0);
        let mut ray = Ray {
            angle: ray_angle_rad,
            pos: Point { x: player_x as f64, y: player_y as f64 }
        };
        let start_pos = Point {
            x: (25.0 * ray.pos.x),
            y: (25.0 * ray.pos.y),
        };

        context.fill_rect(start_pos.x, start_pos.y, 5.0, 5.0);
        step.x = ray.angle.cos();
        step.y = ray.angle.sin();

        while !wall_hit {
            if map_as_i32[ray.pos.y as usize][ray.pos.x as usize] > 0 {
                wall_hit = true;
            } else {
                ray.pos.x += step.x / 25.0;
                ray.pos.y += step.y / 25.0;
                wall_dist.x += step.x;
                wall_dist.y += step.y;
            }
        //cast ray
        context.move_to(start_pos.x + 2.5, start_pos.y + 2.5);
        //calculate destination
        let dest_x = start_pos.x + wall_dist.x;
        let dest_y = start_pos.y + wall_dist.y;
        context.line_to(dest_x, dest_y);
        context.stroke();
    }
    
    //End drawing
    context.close_path();
    Ok(())
}

#[wasm_bindgen]
pub fn roll_dice(sides: f64) -> Result<u16, JsValue> {
    let mut rng = rand::thread_rng();
    let ret_number: u16 = rng.gen_range(1, sides as u16 + 1);
    Ok(ret_number)
}