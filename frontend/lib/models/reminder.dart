import 'dart:convert';

import 'package:intl/intl.dart';

class Reminder {
  String? id;
  String title;
  int due;

  Reminder({
    this.id,
    required this.title,
    required this.due,
  });

  DateTime get dueDate =>
      DateTime.fromMillisecondsSinceEpoch(due * 1000, isUtc: false);

  String get dueDateFmt => DateFormat('dd MMM yyyy').format(dueDate).toString();

  Reminder.fromJson(Map<String, dynamic> json)
      : id = json['id'],
        title = json['title'],
        due = json['due'];

  String toJson() => jsonEncode({
        'id': id,
        'title': title,
        'due': due,
      });
}
