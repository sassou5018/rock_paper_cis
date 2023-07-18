use std::fs;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Can't access file");
    let mut sum_score: u64 = 0;
    for line in data.lines(){
        let letters_vec: Vec<&str> = line.split_whitespace().collect();
        match letters_vec[1]{
            "X"=> sum_score+=1,
            "Y"=> sum_score+=2,
            "Z"=> sum_score+=3,
            _=> println!("Weird bug")
        }
        match is_win(&letters_vec) {
            WinEnum::Draw=> sum_score+=3,
            WinEnum::Win=> sum_score+=6,
            _=>sum_score+=0
        }

    }

    println!("Score: {}", sum_score)
}

enum WinEnum{
    Win,
    Draw,
    Loss
}

fn is_win(played_vec: &Vec<&str>)->WinEnum{
    if played_vec[1] == "X"{
        match played_vec[0] {
            "A" => return WinEnum::Draw,
            "C" => return WinEnum::Win,
            _=> return WinEnum::Loss
        }
    } else if played_vec[1] == "Y"{
        match played_vec[0]{
            "A"=> return WinEnum::Win,
            "B"=> return WinEnum::Draw,
            _=> return WinEnum::Loss
        }
    } else if played_vec[1] == "Z"{
        match played_vec[0] {
            "B"=> return WinEnum::Win,
            "C"=> return WinEnum::Draw,
            _=> return WinEnum::Loss 
        }
    } else {return WinEnum::Loss}
}



// A: Rock, B: Paper, C: Scissors
//X: Rock, Y: Paper, Z: Scissors