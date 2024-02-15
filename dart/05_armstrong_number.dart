class ArmstrongNumbers {
  bool isArmstrongNumber(String strNum) {
    // parse string number to a BigInt
    final number = BigInt.tryParse(strNum);

    // check if number is not a valid
    if (number == null) {
      return false;
    }

    BigInt square = BigInt.zero;

    final length = strNum.length;

    strNum.split("").forEach((x) {
      square += BigInt.parse(x).pow(length);
    });

    return square == number;
  }
}
