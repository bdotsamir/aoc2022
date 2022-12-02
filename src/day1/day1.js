const { readFileSync } = require('fs');

const fileContents = readFileSync("./src/day1/inventory.txt").toString();

const inventoryAsNumbers = fileContents.split("\n\n")
  .map(g => g.split("\n").map(g => +g));

let totalInventories = [];
for (const inventory of inventoryAsNumbers) {
  const reduced = inventory.reduce((acc, val) => acc + val);
  totalInventories.push(reduced);
}

totalInventories.sort((a, z) => a - z).reverse();

//console.log(totalInventories.join(", "));
console.log(totalInventories[0], totalInventories[1], totalInventories[2]);
console.log(totalInventories[0] + totalInventories[1] + totalInventories[2]);
