import 'dart:convert';

import 'package:reminders/models/reminder.dart';
import 'package:http/http.dart' as http;
import 'package:package_info_plus/package_info_plus.dart';

const baseurl = String.fromEnvironment('REMINDERS_BASE_URL',
    defaultValue: 'http://localhost:9999');

const accessToken = String.fromEnvironment("AUTH_TOKEN");

class RemindersApiNotAuthorizedException implements Exception {}

Future<Uri> get url async {
  final info = await PackageInfo.fromPlatform();
  final version = info.version.split(".").first;
  if (version == "1") {
    return Uri.parse('$baseurl/reminders/');
  }
  return Uri.parse('$baseurl/reminders/v$version/');
}

Future<void> create(Reminder reminder) async {
  final response = await http.post(
    await url,
    headers: {
      'Content-Type': 'application/json',
      'Authorization': 'Bearer $accessToken',
    },
    body: reminder.toJson(),
  );

  if (response.statusCode == 401) {
    throw RemindersApiNotAuthorizedException();
  }

  if (response.statusCode != 201) {
    throw Exception('Failed to create reminder.');
  }
}

Future<List<Reminder>> get() async {
  final response = await http.get(await url, headers: {
    'Authorization': 'Bearer $accessToken',
  });

  if (response.statusCode == 401) {
    throw RemindersApiNotAuthorizedException();
  }

  if (response.statusCode == 200) {
    final List<dynamic> reminders = jsonDecode(response.body);
    return reminders.map((e) => Reminder.fromJson(e)).toList();
  }

  return [];
}

Future<void> delete(Reminder reminder) async {
  final response = await http.delete(
    await url,
    headers: {
      'Content-Type': 'application/json',
      'Authorization': 'Bearer $accessToken',
    },
    body: reminder.toJson(),
  );

  if (response.statusCode == 401) {
    throw RemindersApiNotAuthorizedException();
  }

  if (response.statusCode != 200) {
    throw Exception('Failed to delete reminder.');
  }
}
