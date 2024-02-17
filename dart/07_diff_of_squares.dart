class DifferenceOfSquares {
  ///
  /// Formula to calculate sum of the first N natural number is:
  /// (num * (num + 1) ~/ 2)
  ///
  int squareOfSum(int num) {
    final sum = (num * (num + 1) ~/ 2);

    return sum * sum;
  }

  ///
  /// Formula to calculate sum of the squares of first N natural number is:
  /// (num * (num + 1) * (2 * num + 1)) ~/ 6
  ///
  int sumOfSquares(int num) => (num * (num + 1) * (2 * num + 1)) ~/ 6;

  int differenceOfSquares(int num) => squareOfSum(num) - sumOfSquares(num);
}
