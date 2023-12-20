import 'package:flutter/material.dart';
import 'package:intl/date_symbol_data_local.dart';
import 'package:reminders/reminders.dart' as reminders;
import 'package:reminders/pages/add_reminder.dart';
import 'package:reminders/models/reminder.dart';

class Home extends StatefulWidget {
  const Home({super.key, required this.title});

  final String title;

  @override
  State<Home> createState() => _HomeState();
}

class _HomeState extends State<Home> {
  List<Reminder> _reminders = [];

  void _getReminders() {
    reminders.get().then((List<Reminder> r) {
      r.sort((a, b) {
        int cmp = a.dueDate.compareTo(b.dueDate);
        if (cmp != 0) return cmp;
        return a.title.compareTo(b.title);
      });

      setState(() {
        _reminders = r;
      });
    });
  }

  @override
  void initState() {
    super.initState();
    initializeDateFormatting();
    _getReminders();
  }

  Widget _buildReminder(BuildContext context, int index) {
    return ListTile(
      title: Text(_reminders[index].title),
      subtitle: Text(_reminders[index].dueDateFmt),
      trailing: IconButton(
        icon: const Icon(Icons.check),
        onPressed: () => reminders.delete(_reminders[index]).then(
              (_) => _getReminders(),
            ),
      ),
    );
  }

  Widget _buildReminders(BuildContext context) {
    return ListView.builder(
      itemCount: _reminders.length,
      itemBuilder: (BuildContext context, int index) {
        return _buildReminder(context, index);
      },
    );
  }

  // Function to have a loading screen while the reminders are being fetched
  Widget _buildLoading() {
    return const Center(
      child: CircularProgressIndicator(),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
        actions: <Widget>[
          IconButton(
            icon: const Icon(Icons.add),
            onPressed: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (context) => const AddReminder(),
                ),
              ).then((_) {
                _getReminders();
              });
            },
          ),
          const Padding(padding: EdgeInsets.only(right: 23.0)),
        ],
      ),
      body: Center(
        child: _reminders.isEmpty ? _buildLoading() : _buildReminders(context),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => {
          setState(() => _reminders = []),
          _getReminders(),
        },
        tooltip: 'Reload',
        child: const Icon(Icons.refresh),
      ),
    );
  }
}
