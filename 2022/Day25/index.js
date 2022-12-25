function toSnafu(number) {
    let snafu = "";

    while (number !== 0) {
        const remainder = number % 5;

        switch (remainder) {
            case 0:
                snafu = "0" + snafu;
                break;
            case 1:
                snafu = "1" + snafu;
                number -= 1;
                break;
            case 2:
                snafu = "2" + snafu;
                number -= 2;
                break;
            case 3:
                snafu = "=" + snafu;
                number += 2;
                break;
            case 4:
                snafu = "-" + snafu;
                number += 1;
                break;
            default:
                break;
        }
        number = Math.floor(number / 5);
    }

    return snafu;
}

function toDecimal(snafu) {
    let sum = 0;

    for (let i = 0; i < snafu.length; i++) {
        const element = snafu[snafu.length - i - 1];

        switch (element) {
            case "2":
                sum += 2 * 5 ** i;
                break;
            case "1":
                sum += 1 * 5 ** i;
                break;
            case "0":
                break;
            case "-":
                sum -= 1 * 5 ** i;
                break;
            case "=":
                sum -= 2 * 5 ** i;
                break;
            default:
                break;
        }
    }

    return sum;
}

export function solve(input) {
    let fuel = 0;
    input.split("\n").forEach((line) => {
        fuel += toDecimal(line);
    });

    console.log("Part 1: " + toSnafu(fuel));
    console.log("Part 2: MERRY CHRISTMAS!");
}
