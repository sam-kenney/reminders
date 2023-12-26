import 'package:flutter/material.dart';
import 'package:intl/date_symbol_data_local.dart';
import 'package:reminders/reminders.dart' as reminders;
import 'package:reminders/pages/add_reminder.dart';
import 'package:reminders/models/reminder.dart';
import 'package:confetti/confetti.dart';

class Home extends StatefulWidget {
  const Home({super.key, required this.title});

  final String title;

  @override
  State<Home> createState() => _HomeState();
}

class _HomeState extends State<Home> {
  final _confetti = ConfettiController(duration: const Duration(seconds: 2));
  List<Reminder> _reminders = [];
  bool _callingApi = false;
  bool _authorized = true;
  bool _all = false;

  void _getReminders() async {
    setState(() {
      _callingApi = true;
    });

    try {
      List<Reminder> r = await reminders.get();

      r.sort((a, b) {
        int cmp = a.dueDate.compareTo(b.dueDate);
        if (cmp != 0) return cmp;
        return a.title.compareTo(b.title);
      });

      List<Reminder> todayToDo = r.where((e) {
        final t = today();
        return (e.dueDate.isBefore(t) || e.dueDate == t);
      }).toList();

      if (!_all) {
        r = todayToDo;
      }

      setState(() {
        _reminders = r;
        _callingApi = false;
      });
    } on reminders.RemindersApiNotAuthorizedException {
      setState(() {
        _authorized = false;
        _callingApi = false;
      });
    }
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
      subtitle: _reminders[index].subtitle(context),
      trailing: IconButton(
        icon: const Icon(Icons.check),
        onPressed: () {
          _confetti.play();
          final Reminder r = _reminders[index];
          _reminders.removeAt(index);
          reminders.delete(r).then(
                (_) => _getReminders(),
              );
        },
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
        backgroundColor: const Color.fromARGB(232, 231, 105, 29),
        title: Text(widget.title),
        actions: <Widget>[
          IconButton(
            icon: _all
                ? const Icon(Icons.calendar_month)
                : Icon(Icons.calendar_today,
                    color: Theme.of(context).primaryColor),
            tooltip: 'Show all reminders.',
            onPressed: () {
              setState(() {
                _all = !_all;
              });
              _getReminders();
            },
          ),
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
      body: _authorized
          ? Center(
              child: RefreshIndicator(
                onRefresh: () async {
                  _getReminders();
                  await Future.delayed(const Duration(seconds: 1));
                },
                child: _callingApi && _reminders.isEmpty
                    ? _buildLoading()
                    : _buildReminders(context),
              ),
            )
          : const Center(
              child: Text('Not authorized'),
            ),
      bottomNavigationBar: ConfettiWidget(
        blastDirectionality: BlastDirectionality.explosive,
        confettiController: _confetti,
        particleDrag: 0.05,
        emissionFrequency: 0.05,
        numberOfParticles: 25,
        gravity: 0.05,
        shouldLoop: false,
      ),
    );
  }
}
