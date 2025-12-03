const start = Date.now();
const endDate = start + 1000; // 1000ms = 1s
while (Date.now() < endDate) {
  // busy wait
}
'loop finished';
