var fs = require('fs');
var readline = require('readline');

var rd = readline.createInterface({
    input: fs.createReadStream('day-1-input-1-test2'),
    output: false,
    console: false
});

const set = {};
let numbers = "zero|one|two|three|four|five|six|seven|eight|nine";

numbers.split("|").forEach((n, index) => {
    set[n] = index;
    set[index.toString()] = index;
})

let re = new RegExp("(?=(zero|one|two|three|four|five|six|seven|eight|nine|[0-9]))", "g");

let acc = 0;

rd.on('line', function(line) {
        let lcLine = line.toLowerCase();
        let captures =  lcLine.match(re);
        console.log(captures);
        let start = captures[0];
        let end = captures[captures.length-1];
        let digit = set[start] * 10 + set[end];
        acc += digit;
        console.log(`line: ${line}, start: ${start}, end: ${end} digit: ${digit} acc: ${acc}`);

});

rd.on('close', function() {
    console.log(acc);
});
