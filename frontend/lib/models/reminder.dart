import 'dart:convert';
import 'package:flutter/material.dart';

import 'package:intl/intl.dart';

DateTime today() {
  final now = DateTime.now().toLocal();
  return DateTime(now.year, now.month, now.day);
}

class Reminder {
  String? id;
  String title;
  int due;
  int priority;
  String? assignee;

  Reminder({
    this.id,
    required this.title,
    required this.due,
    required this.priority,
    this.assignee,
  });

  DateTime get dueDate =>
      DateTime.fromMillisecondsSinceEpoch(due * 1000, isUtc: false);

  String get dueDateFmt => DateFormat('dd MMM yyyy').format(dueDate).toString();

  Text subtitle(BuildContext context) {
    return Text(
      "$dueDateFmt ${assignee != null ? '- $assignee' : ''}",
      style: TextStyle(
        color: dueDate.isBefore(today())
            ? Colors.red
            : Theme.of(context).textTheme.titleSmall!.color,
      ),
    );
  }

  Reminder.fromJson(Map<String, dynamic> json)
      : id = json['id'],
        title = json['title'],
        due = json['due'],
        priority = json['priority'],
        assignee = json['assignee'];

  Map<String, Object?> toMap() => {
        'id': id,
        'title': title,
        'due': due,
        'priority': priority,
        'assignee': assignee,
      };

  String toJson() => jsonEncode(toMap());

  @override
  String toString() {
    return 'Reminder(id="$id", title="$title", due="$dueDateFmt", priority=$priority, assignee="$assignee")';
  }
}
