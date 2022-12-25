function mix(numbers) {
    const n = numbers.length - 1;
    for (let i = 0; i < numbers.length; i++) {
        const index = numbers.findIndex(n => n[1] === i);
        const [number, startingIndex] = numbers.splice(index, 1)[0];
        const mixedIndex = (((number + index) % n) + n) % n;
        numbers.splice(mixedIndex, 0, [number, startingIndex]);
    }
    return numbers;
}

function coordinateSum(numbers) {
    const n = numbers.length
    const zero = numbers.findIndex(n => n[0] === 0);
    const x = numbers[((zero + 1000 % n) + n) % n][0];
    const y = numbers[((zero + 2000 % n) + n) % n][0];
    const z = numbers[((zero + 3000 % n) + n) % n][0];

    return x + y + z; 
}

export function solve(input) {
    const numbers = input.split("\n").map(Number).map((number, index) => [number, index]);
    const numbers2 = numbers.map(n => [n[0] * 811589153, n[1]]);
    
    for (let i = 0; i < 10; i++) {
        mix(numbers2);
    }
    
    console.log("Part 1: " + coordinateSum(mix(numbers)));
    console.log("Part 2: " + coordinateSum(numbers2));
}