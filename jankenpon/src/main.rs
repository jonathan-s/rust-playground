#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
/*

*/

use text_io::read;
use std::option;

enum Tool {
    Stone,
    Scissors,
    Paper
}


struct PlayerChoice {
    tool: Tool
}

impl PlayerChoice {
    fn new(name: &str) -> PlayerChoice {
        let tool: Tool = match name {
            "stone" => Tool::Stone,
            "scissors" => Tool::Scissors,
            "paper" => Tool::Paper,
            _ => panic!("Invalid choice")
        };
        PlayerChoice {tool: tool}
    }

    fn calculate_points(choice_one: &PlayerChoice, choice_two: &PlayerChoice) -> (u32, u32) {
        let clash = (&choice_one.tool, &choice_two.tool);

        let result: (u32, u32) = match clash {
            (Tool::Stone, Tool::Paper) => (0, 1),
            (Tool::Stone, Tool::Scissors) => (1, 0),
            (Tool::Paper, Tool::Scissors) => (0, 1),
            (Tool::Paper, Tool::Stone) => (1, 0),
            (Tool::Scissors, Tool::Paper) => (1, 0),
            (Tool::Scissors, Tool::Stone) => (0, 1),
            _ => (0, 0)
        };
        result

    }
}

struct Player<'a> {
    name: & 'a str,
    points: u32,
    choice: Option<PlayerChoice>  // should be optional?
}

impl Player<'_> {
    fn new(name: &str) -> Player {
        Player { name: name, points: 0, choice: None}

    }

    fn award_points(player_one: &mut Player, player_two: &mut Player) {
        let result = PlayerChoice::calculate_points(
            player_one.choice.as_ref().unwrap(),
            player_two.choice.as_ref().unwrap()
        );
        let (self_points, other_points);
        (self_points, other_points) = result;
        player_one.points = player_one.points + self_points;
        player_two.points = player_two.points + other_points;
    }

    fn choose(&mut self, choice: &str) {
        self.choice = Some(PlayerChoice::new(choice));
    }
}

struct Game {
    matches: u32
}

impl Game {
    fn new(matches: u32) -> Game {
        Game { matches: matches}
    }

    fn get_winner<'a>(player_one: &'a mut Player<'a>, player_two: &'a mut Player<'a>) -> Option<&'a mut Player<'a>> {
        if player_one.points > player_two.points {
            Some(player_one)
        } else if player_two.points > player_one.points {
            Some(player_two)
        } else {
            None
        }
    }

    fn run(self) {
        let mut current_match: u32 = 0;
        print!("Name your player one: ");
        let player_1_name: String = read!();
        let mut player_one = Player::new(player_1_name.as_str());
        print!("\nName your player two: ");
        let player_2_name: String = read!();
        let mut player_two = Player::new(player_2_name.as_str());
        loop {
            if current_match == self.matches {
                break;
            }
            println!("Pick choice for {}: ", &player_one.name);
            let choice: String = read!();
            player_one.choose(choice.as_str());

            println!("Pick choice for {}: ", &player_two.name);
            let choice: String = read!();
            player_two.choose(choice.as_str());

            Player::award_points(&mut player_one, &mut player_two);
            current_match = current_match + 1;
        }

        let winner = Game::get_winner(&mut player_one, &mut player_two);
        if let Some(i) = winner {
            println!("{} is the winner, they got {} points", i.name, i.points);
        } else {
            println!("No one won, it was even.")
        }
    }
}

struct CliGame(Game);


fn main() {
    let game = Game::new(2);
    game.run();
}
