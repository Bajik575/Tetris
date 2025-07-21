use macroquad::prelude::*;
use std::time::Duration;
use tetris::*;

fn update(game_varriables: &mut GameVarriables) -> Result<(), ()> {
    {
        game_varriables.figure.show(&mut game_varriables.arr);

        // Вывод
        for i in game_varriables.arr.iter() {
            print!("{}", constants::VERTICAL_SYMBOL);
            for j in i {
                print!(
                    "{}",
                    if *j {
                        constants::SYMBOL_1
                    } else {
                        constants::SYMBOL_0
                    }
                )
            }
            println!("{}", constants::VERTICAL_SYMBOL);
        }
        game_varriables.figure.hide(&mut game_varriables.arr);
    }
    // Конец вывода

    // Проверка нужно ли создовать новую фигуру
    // И проверка нужно ли удалять заполненные строки
    if !game_varriables
        .figure
        .clone()
        .falldown()
        .is_valid(&game_varriables.arr)
    {
        for point in game_varriables.figure.points {
            game_varriables.arr[point.1 as usize][point.0 as usize] = true;
        }
        game_varriables.figure = Figure::new();

        // Поиск и удаление заполненных строк
        {
            let mut lines_should_be_shifting: std::collections::HashSet<u8> =
                std::collections::HashSet::new();
            // Поиск строк, которые нужно удалить
            for (line_num, line) in game_varriables.arr.iter().enumerate() {
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
                let mut k = game_varriables.arr.len();
                for i in (0..=game_varriables.arr.len() - 1).rev() {
                    if !lines_should_be_shifting.contains(&(i as u8)) {
                        k -= 1;
                        game_varriables.arr[k] = game_varriables.arr[i];
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

#[macroquad::main("Tetris")]
async fn main() {
    let mut game_varriables = GameVarriables {
        figure: Figure::new(),
        arr:    [[false; constants::WIDTH]; constants::HEIGHT],
        score:  constants::SCORE_INITIAL_VALUE,
    };
    // Основной цикл
    'main :loop {
        let mut last_key_presset = None;
        for _ in 0..100 {
            next_frame().await;
            if let Some(key) = get_last_key_pressed() {
                println!("\n\n\n\n\n");
                last_key_presset = Some(key);
                break;
            }
            std::thread::sleep(Duration::from_millis(constants::DELAY/100));

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
                    if game_varriables.arr[point.1 as usize][(point.0 as i8 + input_vector) as usize] == true {
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
                if game_varriables.figure.clone().rotate().is_valid(&mut game_varriables.arr) {
                    game_varriables.figure.rotate();
                }
            }
            Some(KeyCode::Escape) => break,
            _ => (),
        }
        update(&mut game_varriables);
    }
}
