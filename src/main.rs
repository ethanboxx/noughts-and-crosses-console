use std::io;
mod draw;
mod tests;

fn main() {
    println!("Welcome to my noughts and crosses game made in rust.");
    let mut row1 = [" "," "," "];
    let mut row2 = [" "," "," "];
    let mut row3 = [" "," "," "];

    let mut player_position: [i32; 2] = [1,1];

    println!("Crosses goes first.");

    println!("The board looks like this.");
    draw::draw_game_board(&row1, &row2, &row3, &player_position);
    println!("You are the *");
    loop {
        println!("To move the star left type 4 and hit enter");
        println!("To move the star right type 6 and hit enter");
        println!("To move the star up type 8 and hit enter");
        println!("To move the star down type 2 and hit enter");
        println!("To place your cross type 5 and hit enter");
        let mut movement = String::new();

        io::stdin().read_line(&mut movement)
            .expect("Failed to read line");
        let movement: u32 = match movement.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if movement == 4 {
            if player_position[1] == 1 {
                println!("That move is invalid please try again.");
                continue;
            }
            player_position = [player_position[0] , player_position[1] - 1];
        }
        else if movement == 6 {
            if player_position[1] == 3 {
                println!("That move is invalid please try again.");
                continue;
            }
            player_position = [player_position[0] , player_position[1] + 1];
        }
        else if movement == 8 {
            if player_position[0] == 1 {
                println!("That move is invalid please try again.");
                continue;
            }
            player_position = [player_position[0] - 1 , player_position[1]];
        }
        else if movement == 2 {
            if player_position[0] == 3 {
                println!("That move is invalid please try again.");
                continue;
            }
            player_position = [player_position[0] + 1, player_position[1]];
        }
        else if movement == 0 {
            break;
        }
        else if movement == 5 {
            break;
        }
        else {
            println!("That move is invalid please try again.");
            continue; 
        }
        draw::draw_game_board(&row1, &row2, &row3, &player_position);
    }

    draw::draw_game_board(&row1, &row2, &row3, &player_position);
    let winner = tests::has_someone_won(&row1, &row2, &row3);
    println!("winner {}", winner)
}
