use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use std::time::Duration;
use tetris::*;

fn update(game_varriables: &mut GameVarriables) -> Result<(), ()> {
    // Проверка нужно ли создовать новую фигуру
    // И проверка нужно ли удалять заполненные строки
    if !game_varriables
        .figure
        .clone()
        .falldown()
        .is_valid(&game_varriables.output_arr)
    {
        for point in game_varriables.figure.points {
            game_varriables.output_arr[point.1 as usize][point.0 as usize] = true;
        }
        game_varriables.figure = Figure::new();
        if !game_varriables.figure.is_valid(&game_varriables.output_arr){return Err(())}
        // Поиск и удаление заполненных строк
        {
            let mut lines_should_be_shifting: std::collections::HashSet<u8> =
                std::collections::HashSet::new();
            // Поиск строк, которые нужно удалить
            for (line_num, line) in game_varriables.output_arr.iter().enumerate() {
                let mut full_cells_in_line = 0;
                for cell in line.iter() {
                    if *cell {
                        full_cells_in_line += 1;
                    }
                }
                if full_cells_in_line == constants::WIDTH {
                    lines_should_be_shifting.insert(line_num as u8);
                }
            }
            // Удаление определённых строк и сдвиг строк выше(Если есть заполненные строки)
            if !lines_should_be_shifting.is_empty() {
                // Удаление определённых строк и сдвиг строк выше
                let mut k = game_varriables.output_arr.len();
                for i in (0..=game_varriables.output_arr.len() - 1).rev() {
                    if !lines_should_be_shifting.contains(&(i as u8)) {
                        k -= 1;
                        game_varriables.output_arr[k] = game_varriables.output_arr[i];
                    } else {
                        game_varriables.score += constants::SCORE_ADDITIONAL_VALUE;
                    }
                }
            }
        }
    }

    // Сдвиг фигуры вниз
    game_varriables.figure.falldown();
    Ok(())
}
fn display_output_arr(output_arr: &OutputArr) {
    // Вывод
    for (i, string) in output_arr.iter().enumerate() {
        for (j, b) in string.iter().enumerate() {
            draw_rectangle(
                j as f32 * constants::PIXELS_IN_BLOCK as f32,
                i as f32 * constants::PIXELS_IN_BLOCK as f32,
                constants::PIXELS_IN_BLOCK as f32,
                constants::PIXELS_IN_BLOCK as f32,
                if *b {
                    constants::BLOCK_COLOR
                } else {
                    constants::BACKGROUND_COLOR
                },
            );
        }
    }
}
fn draw_ui(score: i32){
    draw_line(
        0f32,
        (constants::HEIGHT as u32 * constants::PIXELS_IN_BLOCK) as f32,
        (constants::WIDTH  as u32 * constants::PIXELS_IN_BLOCK) as f32,
        (constants::HEIGHT as u32 * constants::PIXELS_IN_BLOCK) as f32,
        3f32,
        Color::from_hex(0x7d7d7d),
    );
    draw_text(
        format!("Score:{}",score).as_str(),
        ((constants::WIDTH as u32 * constants::PIXELS_IN_BLOCK ) /2) as f32 - 76.5625 / 2f32,
        (constants::HEIGHT as u32 * (constants::PIXELS_IN_BLOCK + 1)) as f32 - 2.5f32,
        25f32,
        WHITE);
}
fn display(game_varriables: &mut GameVarriables){
    draw_ui(game_varriables.score);
    
    game_varriables.figure.show(&mut game_varriables.output_arr);
    display_output_arr(&game_varriables.output_arr);
    game_varriables.figure.hide(&mut game_varriables.output_arr);
    
}

#[macroquad::main("Tetris")]
async fn main() {
    let mut game_varriables = GameVarriables {
        figure: Figure::new(),
        output_arr: [[false; constants::WIDTH]; constants::HEIGHT],
        score: constants::SCORE_INITIAL_VALUE,
    };
    // Основной цикл
    set_window_size(constants::WIDTH as u32 * constants::PIXELS_IN_BLOCK,
                    (1 + constants::HEIGHT as u32)  * constants::PIXELS_IN_BLOCK);
    loop {
        let mut last_key_presset = None;
        for _ in 0..100 {
            display(&mut game_varriables);
            next_frame().await;
            
            if let Some(key) = get_last_key_pressed() {
                last_key_presset = Some(key);
                break;
            }
            std::thread::sleep(Duration::from_millis(constants::DELAY / 100));
        }
        match last_key_presset {
            Some(key @ (KeyCode::Left | KeyCode::A | KeyCode::Right | KeyCode::D)) => {
                let input_vector: i8 = match key {
                    KeyCode::Left | KeyCode::A => -1,
                    KeyCode::Right | KeyCode::D => 1,
                    _ => 0,
                };

                // Проверка на возможность смещения
                let mut is_possible_shift = true;
                for point in game_varriables.figure.points.iter() {
                    if point.0 as i8 + input_vector == -1
                        || point.0 as i8 + input_vector == constants::WIDTH as i8
                    {
                        is_possible_shift = false;
                        break;
                    }
                    if game_varriables.output_arr[point.1 as usize]
                        [(point.0 as i8 + input_vector) as usize]
                        == true
                    {
                        is_possible_shift = false;
                        break;
                    }
                }

                // Смещение всех точек фигуры в направлении input_vector
                if is_possible_shift {
                    for point in game_varriables.figure.points.iter_mut() {
                        point.0 = point.0 + input_vector;
                    }
                }
                game_varriables.figure.main_point.0 += input_vector;
            }
            //Поворот при нажатии Enter или ArrowUp
            Some(KeyCode::Enter | KeyCode::Up) => {
                if game_varriables
                    .figure
                    .clone()
                    .rotate()
                    .is_valid(&mut game_varriables.output_arr)
                {
                    game_varriables.figure.rotate();
                }
            }
            Some(KeyCode::Escape) => break,
            _ => (),
        }
        match update(&mut game_varriables){
            Err(()) => break,
            Ok(())  => (),
        };
    }
}