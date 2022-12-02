const { readFileSync } = require('fs');

const fileContents = readFileSync("./inventory.txt").toString();

const inventoryAsNumbers = fileContents.split("\n\n")
  .map(g => g.split("\n").map(g => +g));

let max = 0;
for (const inventory of inventoryAsNumbers) {
  const reduced = inventory.reduce((acc, val) => acc + val);
  if (reduced > max) max = reduced;
}

console.log(max);
