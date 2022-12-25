function win(me, opponent) {
    return (
        (me === "rock" && opponent === "scissors") ||
        (me === "paper" && opponent === "rock") ||
        (me === "scissors" && opponent === "paper")
    );
}

const shape_to_points = {
    rock: 1,
    paper: 2,
    scissors: 3,
};

const me_to_shape = {
    X: "rock",
    Y: "paper",
    Z: "scissors",
};

const me_to_outcome = {
    X: "lose",
    Y: "draw",
    Z: "win",
};

const opponent_to_shape = {
    A: "rock",
    B: "paper",
    C: "scissors",
};

const who_wins = {
    rock: "paper",
    paper: "scissors",
    scissors: "rock",
};

const who_looses = {
    rock: "scissors",
    paper: "rock",
    scissors: "paper",
};

export function solve(input) {
    let rounds = input.trim().split("\n");

    let score1 = 0;
    let score2 = 0;
    rounds.forEach((element) => {
        let [opponent, me] = element.split(" ");

        let opponent_shape = opponent_to_shape[opponent];
        let my_shape = me_to_shape[me];

        score1 += shape_to_points[my_shape];
        if (my_shape === opponent_shape) {
            score1 += 3;
        } else if (win(my_shape, opponent_shape)) {
            score1 += 6;
        } else {
            score1 += 0;
        }

        let outcome = me_to_outcome[me];
        if (outcome === "draw") {
            score2 += 3 + shape_to_points[opponent_shape];
        } else if (outcome === "win") {
            score2 += 6 + shape_to_points[who_wins[opponent_shape]];
        } else {
            score2 += shape_to_points[who_looses[opponent_shape]];
        }
    });

    console.log("Part 1: " + score1);
    console.log("Part 2: " + score2);
}
