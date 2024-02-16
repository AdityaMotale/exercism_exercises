class HighScores {
  final List<int> scores;

  const HighScores(this.scores);

  int latest() => scores.last;

  int personalBest() => _sortList().first;

  List<int> personalTopThree() =>
      _sortList().toList().sublist(0, scores.length < 3 ? scores.length : 3);

  List<int> _sortList() => List.from(scores)..sort((a, b) => b.compareTo(a));
}
