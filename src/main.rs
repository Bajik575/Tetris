use console::{Key, Term};
use std::sync::mpsc;
use std::time::Duration;
use tetris::*;
// use std::rc::Rc;
// use std::cell::RefCell;

fn clear_last_lines(term: &Term, n: usize) {
    term.move_cursor_up(n).unwrap();
    for _ in 0..n {
        term.clear_line().unwrap();
    }
}

fn main() {
    println!(
        "{}{}{}",
        constants::NAME_DECORATOR.repeat(((constants::WIDTH + 2) - constants::NAME_LEN) / 2),
        constants::NAME,
        constants::NAME_DECORATOR.repeat(((constants::WIDTH + 2) - constants::NAME_LEN) / 2)
    );
    // Переменные
    let mut score = constants::SCORE_INITIAL_VALUE;
    let mut figure = Figure::new();
    let mut game_over = false;
    let mut arr = [[false; constants::WIDTH]; constants::HEIGHT];

    // Переменные необходимые для работы потока
    let (input_s, input_r) = mpsc::channel::<Key>();
    let term = Term::stdout();

    // Создание потока для получения и отправки ввода
    std::thread::spawn(move || {
        loop {
            input_s.send(term.read_key().unwrap()).unwrap();
        }
    });

    let term = Term::stdout();

    // Резервирование места в консоли для отображения
    print!("{}", "\n".repeat(constants::HEIGHT + 3));

    // Основной цикл
    'a: loop {
        //очистка старого содержимого
        clear_last_lines(&term, constants::HEIGHT + 3);

        // Вывод в консоль
        {
            figure.show(&mut arr);

            // Вывод верхней части рамки
            println!(
                "{}{}{}",
                constants::LEFT_UP_SYMBOL,
                constants::HORIZONTAL_SYMBOL
                    .to_string()
                    .repeat(constants::WIDTH),
                constants::RIGHT_UP_SYMBOL
            );

            // Выход из игры при выполнении условия
            if game_over {
                println!("{}", constants::GAME_OVER_TEXT);
                std::thread::sleep(Duration::from_millis(constants::GAME_OVER_DELAY));
                break 'a;
            }

            // Вывод игрового поля
            for i in arr.iter() {
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
            // Вывод нижней части рамки
            println!(
                "{}{}{}",
                constants::LEFT_DOWN_SYMBOL,
                constants::HORIZONTAL_SYMBOL
                    .to_string()
                    .repeat(constants::WIDTH),
                constants::RIGHT_DOWN_SYMBOL
            );
            println!("{}Score: {}"," ".repeat((constants::WIDTH + 2 - (format!("Score: {}",score.to_string()).len()))/2),score);
            // Удаление точек фигуры из массив вывода
            figure.hide(&mut arr);
        }
        // Конец вывода в консоль

        // Проверка нужно ли создовать новую фигуру
        // И проверка нужно ли удалять заполненные строки
        if !figure.clone().falldown().is_valid(&arr) {
            for point in figure.points {
                arr[point.1 as usize][point.0 as usize] = true;
            }
            figure = Figure::new();
            if !figure.is_valid(&arr) {
                game_over = true;
            }

            // Поиск и удаление заполненных строк
            {
                let mut lines_should_be_shifting: std::collections::HashSet<u8> =
                    std::collections::HashSet::new();
                // Поиск строк, которые нужно удалить
                for (line_num, line) in arr.iter().enumerate() {
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
                    let mut k = arr.len();
                    for i in (0..=arr.len() - 1).rev() {
                        if !lines_should_be_shifting.contains(&(i as u8)) {
                            k -= 1;
                            arr[k] = arr[i];
                        } else {
                            score += constants::SCORE_ADDITIONAL_VALUE;
                        }
                    }
                }
            }
        }

        // Сдвиг фигуры вниз
        figure.falldown();

        // Обработка ввода
        match input_r.recv_timeout(Duration::from_millis(constants::DELAY)) {
            Ok(key @ (Key::ArrowLeft | Key::Char('a') | Key::ArrowRight | Key::Char('d'))) => {
                let input_vector: i8 = match key {
                    Key::ArrowLeft | Key::Char('a') => -1,
                    Key::ArrowRight | Key::Char('d') => 1,
                    _ => 0,
                };

                // Проверка на возможность смещения
                let mut is_possible_shift = true;
                for point in figure.points.iter() {
                    if point.0 as i8 + input_vector == -1
                        || point.0 as i8 + input_vector == constants::WIDTH as i8
                    {
                        is_possible_shift = false;
                        break;
                    }
                    if arr[point.1 as usize][(point.0 as i8 + input_vector) as usize] == true {
                        is_possible_shift = false;
                        break;
                    }
                }

                // Смещение всех точек фигуры в направлении input_vector
                if is_possible_shift {
                    for point in figure.points.iter_mut() {
                        point.0 = point.0 + input_vector;
                    }
                }
                figure.main_point.0 += input_vector;
            }
            //Поворот при нажатии Enter или ArrowUp
            Ok(Key::Enter | Key::ArrowUp) => {
                if figure.clone().rotate().is_valid(&mut arr) {
                    figure.rotate();
                }
            }
            Ok(Key::Escape) => break,
            _ => (),
        }
    }
}
