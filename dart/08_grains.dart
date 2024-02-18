BigInt square(final int n) {
  if (n < 1 || n > 64) {
    throw ArgumentError("square must be between 1 and 64");
  }

  BigInt grainsOnCurrentSquare = BigInt.one;

  for (int square = 1; square < n; square++) {
    grainsOnCurrentSquare *= BigInt.two;
  }

  return grainsOnCurrentSquare;
}

BigInt total() {
  BigInt sum = BigInt.zero;
  BigInt grainsOnCurrentSquare = BigInt.one;

  for (int square = 1; square <= 64; square++) {
    sum += grainsOnCurrentSquare;
    grainsOnCurrentSquare *= BigInt.two;
  }

  return sum;
}
