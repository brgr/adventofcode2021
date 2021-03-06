use inputs;

const EXAMPLE1: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7";
const INPUT: &str = "25,8,32,53,22,94,55,80,33,4,63,14,60,95,31,89,30,5,47,66,84,70,17,74,99,82,21,35,64,2,76,9,90,56,78,28,51,86,49,98,29,96,23,58,52,75,41,50,13,72,92,83,62,37,18,11,34,71,91,85,27,12,24,73,7,77,10,93,15,61,3,46,16,97,1,57,65,40,0,48,69,6,20,68,19,45,42,79,88,44,26,38,36,54,81,59,43,87,39,67\n\n25 83 15 27 22\n97 81 12 80 52\n65 58 91 23 36\n77 60 49 43 95\n13 21 56 78 99\n\n43 85 82 52 40\n19 14 91  4  7\n 6 87 64 26 56\n94 58 81 98 90\n18 72 23 37 20\n\n 5 43 13 47 93\n25 78 64 56 10\n75 90 50 72 14\n 9 29 58 79 62\n69 66 88 35 16\n\n73 13 35 91 24\n66 37 39 88  7\n55 36 41 81 85\n29 49 70  4 23\n98 69 84 56 87\n\n30 44 76 87 57\n67 24 63 38 99\n41 69 35 60 78\n32  6 88  9 55\n46 75 77 13 22\n\n29 68 51 92 64\n50 65 86 22 98\n66  6 44 53 87\n54 63 25 59 14\n96 52 90 58 71\n\n68 73 79 50 98\n10 94 53 46 88\n59 78 35 71 15\n42 67 82 17 92\n40  1 83 61 16\n\n44 64 97 63 93\n32 84 55 70 24\n29 67 68 86 18\n90 72 42 21 88\n53 40 85 27 11\n\n45 31  6 30  3\n66 16 73 65 25\n48 41 98 27 55\n39 17 19  8 24\n85  2 28 63 13\n\n86  3 20 41 53\n98 45 63 74 47\n87 68 97 13 42\n58 73 48 35 17\n91 38 55  4 34\n\n13 44 26 77 23\n51 88 41 42 93\n12 56 38 87 73\n60 19 55 99 21\n 3 34 20 94 32\n\n26 72 39 21 76\n94 12  1 49 60\n38 20 30 48 98\n53 62 22 92 69\n 7 88 57 81 13\n\n99 20 70 52 12\n23 28 45 66 41\n92  8 55 22 36\n14 78  9 46 18\n21 87 89 27 68\n\n76 92 45 53 98\n56 91 72 19 35\n25 59 42 90 68\n47 97 30 65 95\n94 60 52 36 54\n\n 9 18 39 89 29\n25 84 37 72 28\n17 70 27 93  0\n80 36 74 35 71\n11 49 57 46  4\n\n37 66 54 93 77\n40 95 94 34 11\n35 64 92 16 43\n 9  6 83 32 29\n 2 80 10 45 72\n\n37 95 70 62  1\n58 14 38 22 63\n44  7 78 34 39\n73 50 26  0 52\n60 69 87 27 97\n\n10  9 83 11  5\n33 62 18 75 47\n 3 86 36 26 91\n39 80 14 67 15\n74 95 88 37 57\n\n 7 83 44 24 66\n67 60 51 52 46\n27 77 35 72 88\n22 69  1 78 64\n41 58 81 21  3\n\n68 34 11 40 17\n15 43  9 64 49\n32 37 20 14 81\n 3 87 72 16 51\n25 77 58 10 52\n\n89 61 97 14 56\n32 90 98 69  4\n88 58 51 76 66\n15 62 35  7 29\n95  8 33 73 22\n\n25 59 40 71  8\n36 42 47 67 19\n93 50 80 98 79\n72 97 68 81 39\n56 91 12 95 53\n\n65 90 44 88 66\n43 23 35 18 77\n 9 97 16 38 22\n81 49 39 10 41\n36 56 13 29 37\n\n53 32  6 41  8\n 4 38 88 29 37\n58 54 15 83 12\n13  1 98 85 23\n69 49 26 64 70\n\n66 33 15  7 77\n26 16 79 28 58\n69 96 14 44 61\n43 75  0 97 36\n59 41 22 24 87\n\n90 54  4 62 63\n 2 79 59  6 82\n53 74 65 86 75\n71 32 13 80 10\n17  0 20 69 50\n\n60  3 78  2 47\n44 32 23 42 17\n35 59 50 74 54\n64 49 51  5 65\n21 13 63 43 38\n\n 6 53 57 18 33\n26 31  9 44 34\n81 21 39  2 52\n95  5 43 46 91\n98 71 59 30 48\n\n13 31 91  1 67\n96 35 20 19 40\n87 27 78  9 22\n11 45 38 46 51\n72 68 23 25 85\n\n99 97 85 86 20\n92 16 60  6 67\n18 87 93 79 53\n 0 51 56 19 95\n78 84 40 98 34\n\n91 11  1 36 47\n43 62 27 32 50\n75 52 87 29 30\n61 34 39 68 58\n77 18 21 13 40\n\n22 41 63 28 81\n37 39 29 95 83\n49 10 94  0 54\n96 38 80 87  1\n15 93 99 47 23\n\n22 97 54 89 55\n52 63 78 57 84\n47 36 64 21 20\n45 41 16 11 66\n 3 98 10 99  1\n\n26 15 89 54 86\n10 60 52 64 74\n40 91 24 51 66\n95 43 29 34 85\n88 18 97 31 53\n\n61 96 63 89 12\n57 28 29 23 53\n82 40 56 44 13\n50 73  0 30  4\n79 78 64 37 26\n\n29 60 24 73 38\n69 94  6  9  1\n97 40 27 26 86\n59 52  4 15 96\n61 63 55 66 85\n\n98 39 56 63 58\n54 88 41 48 65\n85 28 14 29  2\n20 70 46 72 93\n75 59 36 57 71\n\n38 27 60 37 44\n98  9 13 45 57\n 4 76 33  8 21\n19  7 77 50 22\n71 35 80 46 20\n\n88 73 59 65 41\n61 63 33 85 22\n76 50 19 77 45\n52 99  2  8 83\n25 92 98 60 71\n\n49 40 35 83 36\n15 71 90 47 19\n34 59 55 42 21\n69  7 23  9 70\n43 22 48 57 60\n\n33  4 38 26 59\n50 47 63 75 19\n11 65 24 87 21\n45 16 97 40 57\n83 96 70 41 12\n\n46 82 87 88  9\n51 64 97  6 41\n24 72 79 43 90\n74 92 45 22 54\n 1 95 80 55 14\n\n86 52 90 19 85\n25 67 30 84 56\n66 71 39 74 96\n93 46 89 72 29\n97 40 99 62 44\n\n82 87 79 63  1\n27 61 30 26  6\n76 59 56 44 36\n72 12 88 92 33\n93 78 66 67  9\n\n96 81 75 42 20\n87 13 35 79 77\n 6 31 44 24 80\n32 63 78  2 56\n 1 46 40 99 14\n\n55 24 10 61 89\n 7 37 19 20 60\n68 65 39 18 86\n90 59 79 84 88\n81 74 27 70 73\n\n19 35 91 14 53\n85 89  4 39 70\n80 36  2 57 61\n63 82 81 22 78\n37 43 83 12 94\n\n70 99 79 92 36\n21 30 88 22 96\n11 60 23 41 64\n81 10 13 51 19\n34 45 42 17 38\n\n39 21 37  3  2\n54 32 25 26 81\n98 55 53 35 67\n90 48 15 18 68\n22 78 83 30 72\n\n32 50 94 51 26\n 1 82 86 75 89\n27  6 16 57  3\n91 66 30 39 25\n 9 46 88 12 35\n\n14 91 16 30 45\n41 82 42 26 15\n43 72 81 38 92\n95 87  7 28 46\n63 71 11 22 56\n\n45 58 68 37 81\n16 20 71 82 28\n85 89 23 65 18\n40 66 11 70 10\n60 97 69 86 19\n\n16 47 46 53 13\n48 76 98 66 12\n79 43 25 36 31\n85  1 41  3 50\n99 73 83 89 64\n\n27 82 33 36 83\n73 31 34  7 30\n98 20 32 39 92\n56 90 85 11 23\n 6 89 44 87 50\n\n18 58 84 47 15\n63 16 22 65 72\n82  4 55 13  8\n19 86 11 52  3\n54 80 39 97 12\n\n73 85  3 24 37\n 4  7 75 16 42\n92 95 69 81 66\n 0 40 12 18 49\n26 38 56 25 35\n\n66 91 90 41 44\n89 47 23 24 18\n 6 38 62  2 60\n 1 29  8 53 70\n76 50 85 34 81\n\n 8 99 34 19 80\n46  3 17 26 54\n95 43 63 49 14\n90 77  1 42 85\n83 59 57 33 30\n\n75 12  7 21 70\n89 36 96 46 90\n37 28 23 32 39\n 2 18 81 11 57\n15 24  0  9 65\n\n 7 53  6 34 20\n32 76 24 56 29\n43 62 26 75 72\n94 79 77 60 12\n58 19 17 55  9\n\n35 85 48 30 53\n 3  0 98 74 37\n55 29 81 86  2\n22  7 33 62 94\n 1 31 99 16 14\n\n46 68  6 94 79\n86 99 44 38 91\n93 80 90 50 63\n 2 71 65 23 39\n43 31 20 82 28\n\n20 64 92  2 23\n87 28 99 93 59\n70 30 39 33 51\n13 27 95 90 29\n24 47 83 48 60\n\n64 93 47 22 27\n 7 74 75 26 60\n83  9  5 90 55\n28 57 45 56 98\n21 77 80  8 67\n\n 1 11 79 36 24\n27 37 50 69 98\n 4 39 38  6 59\n49 53 22 31 15\n93 47 86 72 40\n\n14 26 39 20 32\n93 89 19 67 92\n15 16 96 50 51\n 2 86 97 54  5\n25  8 72  4  1\n\n32 64 27 13 63\n70 36 95  9 80\n 2 76 10 16  0\n52 18 12 97 33\n71 82 72 15 99\n\n57 82 29  0 83\n68 33 31 21 60\n 7  2 27 44 89\n15 88 71 70 52\n97  3 63 66 59\n\n45 94 12 48 24\n 2 38 69  6 31\n44 99 52 27 43\n13 74 10 67 76\n35 20 25 86 19\n\n18 26 30 38 32\n 0 27 82 55 72\n53 20 19 58 84\n80 76  2 97  4\n61 24  3 73 92\n\n91 85 95 12 11\n94 49 41 31 47\n98  9 56 55  3\n42 22 19 72 68\n59 54 88 50 16\n\n51 68 98 11 48\n45 17 81 10 94\n38 69 42 40 67\n 1 20 12 27 32\n 8 44 41 79 62\n\n47 65 41 60 12\n92 43 94  1 86\n18 63 26 46 71\n62 21 11 80 98\n23 40 67 77 89\n\n78 67 20 48 53\n99 10 38 51  7\n62 89 87 68 93\n31 55 80 69 29\n36 74 88 44 11\n\n39 27 82 95 52\n53 75 34 35 41\n 0 94 30 62 13\n20 77  2  8 12\n44 32 68 17 99\n\n37 48  9 29 94\n34 23 66 93 86\n33 10 87 61 20\n 1 41 35 80 19\n83 96 47  2 76\n\n62 25  0 47 39\n96 24 99 73 61\n51 72  9 21 20\n97 71 19 83 78\n46 34 44 48  1\n\n60 63 97 56 96\n 1 11 70 59  4\n21 43  8 36 46\n80 88 76 68 37\n86  5 12 15 73\n\n90 94 39 24 89\n71 31 10 51 97\n16 54 52 36 98\n48  7 77 84 57\n88  9 92  0 66\n\n43 45 33  1 26\n56 22  8 78 92\n60 51 96  7 58\n84 31 88 12 73\n76 25 63 87 37\n\n68 62 15 30 18\n 5 49 23 13 73\n45 67 50 35 86\n85 31 53 27 32\n 4  1 90 10  2\n\n13 23 41 82 40\n69 12 17  0 34\n 4 91 71 21 67\n53 87 36 80  6\n83 25 92 29 56\n\n61 95 19 53 22\n50 40 66 58 79\n92 33 47 45 14\n54 32 12 48 78\n89 28 82 80 21\n\n18 35 15 51 50\n41 29 46 22 79\n34 97 92 75 87\n99 76 42  6 58\n86 10 91 21 67\n\n51 27 94 66 64\n83 26 45 87 41\n61 77 68 17 99\n74 93 19 28 50\n12 69 44 63 10\n\n15 10 47 79 12\n90 20 18 19 64\n61  6 33 29 52\n92 37 43 49 13\n91  4 50 65 53\n\n35 80 88 72 81\n22 84 51 96 25\n 4 47 70 27 36\n62 54 78 11  1\n 5  0  6 19 53\n\n83 54  4 26 86\n63 11 50 46 96\n58 99 23 18 82\n14 57 77 98 72\n43 34 25 65  1\n\n73 58 62  8 61\n 0 13 16 82 79\n67 37 93 30 31\n27  7 59 15 72\n68 88 81 49 60\n\n72 56 70 24 18\n53 91 95 11 65\n63 67 49 22 74\n59 25 94 20 97\n 6 73 52 47 38\n\n18 46 93 87 51\n24 28 10 30 38\n82  2 40 17 76\n81 39 97 48  5\n19  0 27 74 63\n\n59 95 47 41 28\n31 57 15  5 40\n21 72 56 99 17\n37 52 27 48 33\n50 53  8 73 68\n\n10 21 79 71  5\n40 92 54 97 46\n45 15  9 42 50\n68 81 90 47 99\n44  2 64 27 69\n\n 4 23 59 88 60\n39 16 28 56 90\n94 78 57 53 46\n20 14 51 44 99\n91 17  7 83 84\n\n74 19 24 39 16\n44 62 61 99 42\n65 63 50 78 38\n27 49 86 80 33\n66 30  2 31 83\n\n87 45 18 99  4\n89 78 27 90 34\n72  6 46 16 57\n60 41 33 82 62\n48 20 55 32 14\n\n97 20 60 49 50\n35  4 90 67 52\n66 72 92 13 30\n85 41 62 77 16\n64 22 40 51 43\n\n31 25 67  3 56\n 1 60 89 98 15\n73 24 41 35 12\n26 83 62 17 79\n95 65 84 14  0\n\n15 71 52 81 45\n99 48 65 79  4\n85 36 37 87 64\n61 95  6 27 34\n17 14 43 60 92\n\n59 93 18 63 19\n92 14 61 13 26\n39 70  2 58  6\n68 57 89 81  4\n55 98 79 85  3";

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct BoardNumber {
    number: u32,
    marked: bool,
}

type Row = Vec<BoardNumber>;
type Board = Vec<Row>;

#[derive(Debug)]
struct Game {
    winning_numbers: Vec<u32>,
    boards: Vec<Board>,
}

fn parse(input: &str) -> Game {
    let x: Vec<&str> = input.split("\n\n").collect();

    let winning_numbers: Vec<u32> = x[0].split(",").map(|s| s.parse::<u32>().unwrap()).collect();
    let boards: Vec<Vec<&str>> = x[1..].iter()
        .map(|s|
            s.split("\n").collect())
        .collect();

    let boards: Vec<Board> = boards.iter().map(|b|
        b.iter().map(|row|
            row.replace("  ", " ")
                .split(" ")
                .filter(|ss| !ss.is_empty())
                .map(|ss| BoardNumber { number: ss.parse::<u32>().unwrap(), marked: false })
                .collect())
            .collect())
        .collect();

    Game {
        winning_numbers,
        boards,
    }
}

fn mark_numbers(boards: &Vec<Board>, winning_number: &u32) -> Vec<Board> {
    boards.iter()
        .map(|board| mark_numbers_for_board(winning_number, board))
        .collect::<Vec<Board>>()
}

fn mark_numbers_for_board(winning_number: &u32, board: &Board) -> Board {
    board.iter()
        .map(|row| row.iter()
            .map(|n|
                if n.number == *winning_number {
                    BoardNumber { marked: true, ..*n }
                } else {
                    BoardNumber { ..*n }
                })
            .collect::<Vec<BoardNumber>>())
        .collect()
}

fn calculate_sum_unmarked(board: &Board) -> u32 {
    let mut sum_unmarked = 0;
    for row in board {
        for x in row {
            if !x.marked { sum_unmarked += x.number; }
        }
    }

    sum_unmarked
}

fn check_board(board: &Board) -> Option<u32> {
    // rows
    for row in board {
        let mut all_marked = true;

        for v in row {
            if !v.marked { all_marked = false; }
        }

        if all_marked { return Some(calculate_sum_unmarked(&board)); }
    }

    // columns
    for i in 0..board[0].len() {
        let mut all_marked = true;

        for row in board {
            if !row[i].marked { all_marked = false; }
        }

        if all_marked { return Some(calculate_sum_unmarked(&board)); }
    }

    None
}

fn check_for_winner(boards: &Vec<Board>) -> Option<u32> {
    for board in boards {
        if let Some(score) = check_board(&board) {
            return Some(score);
        }
    }

    None
}

fn check_for_winner_boards(boards: &Vec<Board>) -> Vec<WinnerBoard> {
    let mut winner_boards = Vec::new();

    for board in boards {
        if let Some(score) = check_board(&board) {
            winner_boards.push(WinnerBoard { board: (*board).clone(), score });
        }
    }

    winner_boards
}

fn part1(game: &mut Game) {
    for winning_number in &game.winning_numbers {
        game.boards = mark_numbers(&mut game.boards, winning_number);

        match check_for_winner(&game.boards) {
            Some(sum_unmarked) => {
                println!("{}", sum_unmarked * winning_number);
                return;
            },
            None => {}
        }
    }
}

#[derive(Debug)]
struct WinnerBoard {
    board: Board,
    score: u32,
}

fn part2(game: &mut Game) {
    let mut winning_boards: Vec<WinnerBoard> = Vec::new();

    for winning_number in &game.winning_numbers {
        game.boards = mark_numbers(&mut game.boards, winning_number);

        let winner_boards = check_for_winner_boards(&game.boards);

        for winner_board in winner_boards {
            let index = get_index_of_element(&game, &winner_board);

            winning_boards.push(WinnerBoard {
                board: winner_board.board,
                score: winner_board.score * winning_number,
            });

            game.boards.remove(index);
        }

        println!("game boards len = {}", game.boards.len());
        if game.boards.is_empty() { break; }
    }

    println!("winning boards: {:?}", winning_boards);

    println!("{}", winning_boards.last().unwrap().score);
    println!("winning boards.len() = {}", winning_boards.len());
}

fn get_index_of_element(game: &&mut Game, winner_board: &WinnerBoard) -> usize {
    game.boards.iter().position(|x| x == &winner_board.board).unwrap()
}

fn main() {
    // let inputs: Vec<String> = inputs::get_input_split(3);
    let mut game = parse(INPUT);
    println!("{:?}", game);

    // part1(&mut game);
    part2(&mut game);

    println!("boards.len() = {}", game.boards.len());

    // println!("{:?}", inputs);
    //
    // part1(inputs);
}
