import 'package:flutter/material.dart';
import 'package:reminders/reminders.dart' as reminders;
import 'package:reminders/models/reminder.dart';
import 'package:intl/date_symbol_data_local.dart';

class AddReminder extends StatefulWidget {
  const AddReminder({super.key});

  @override
  State<AddReminder> createState() => _AddReminderState();
}

class _AddReminderState extends State<AddReminder> {
  @override
  void initState() {
    super.initState();
    initializeDateFormatting();
  }

  final _formKey = GlobalKey<FormState>();
  final _titleController = TextEditingController();
  DateTime selectedDate = DateTime.now();

  Future<void> _selectDate(BuildContext context) async {
    final DateTime? picked = await showDatePicker(
        context: context,
        initialDate: selectedDate,
        firstDate: DateTime(2015, 8),
        lastDate: DateTime(2101));
    if (picked != null && picked != selectedDate) {
      setState(() {
        selectedDate = picked;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Add Reminder'),
      ),
      body: Form(
        key: _formKey,
        child: SingleChildScrollView(
          child: Container(
            margin: const EdgeInsets.all(20),
            child: Column(
              children: <Widget>[
                Row(
                  children: <Widget>[
                    Expanded(
                      child: TextFormField(
                        style: const TextStyle(
                          fontSize: 24,
                        ),
                        controller: _titleController,
                        decoration: const InputDecoration(
                          hintText: 'Task',
                        ),
                        validator: (value) {
                          if (value == null || value.isEmpty) {
                            return 'Please enter a task.';
                          }
                          return null;
                        },
                      ),
                    ),
                    // const SizedBox(height: 20),
                    IconButton(
                      onPressed: () => _selectDate(context),
                      icon: const Icon(Icons.calendar_today),
                      tooltip: 'Select due date',
                      hoverColor: Colors.pink.shade100,
                    )
                  ],
                ),
                const SizedBox(height: 20),
                ElevatedButton(
                  onPressed: () {
                    if (_formKey.currentState == null) {
                      return;
                    } else if (_formKey.currentState!.validate()) {
                      final reminder = Reminder(
                        title: _titleController.text,
                        due: selectedDate.millisecondsSinceEpoch ~/ 1000,
                      );
                      reminders.create(reminder).then((_) {
                        Navigator.pop(context);
                      });
                    }
                  },
                  style: ElevatedButton.styleFrom(
                    minimumSize: const Size(200, 60),
                  ),
                  child: const Text('Submit'),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
