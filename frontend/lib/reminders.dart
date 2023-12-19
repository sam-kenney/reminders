import 'dart:convert';

import 'package:reminders/models/reminder.dart';
import 'package:http/http.dart' as http;

const String url = 'https://reminders-api-gwptt77ggq-ts.a.run.app/reminders/';

Future<void> create(Reminder reminder) async {
  final response = await http.post(
    Uri.parse(url),
    headers: {
      'Content-Type': 'application/json',
    },
    body: reminder.toJson(),
  );

  if (response.statusCode != 201) {
    throw Exception('Failed to create reminder.');
  }
}

Future<List<Reminder>> get() async {
  final response = await http.get(Uri.parse(url));

  if (response.statusCode == 200) {
    final List<dynamic> reminders = jsonDecode(response.body);
    return reminders.map((e) => Reminder.fromJson(e)).toList();
  }

  return [];
}

Future<void> delete(Reminder reminder) async {
  final response = await http.delete(
    Uri.parse(url),
    headers: {
      'Content-Type': 'application/json',
    },
    body: reminder.toJson(),
  );

  if (response.statusCode != 200) {
    throw Exception('Failed to delete reminder.');
  }
}
