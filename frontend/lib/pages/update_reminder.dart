import 'package:flutter/material.dart';
import 'package:reminders/reminders.dart' as reminders;
import 'package:reminders/models/reminder.dart';
import 'package:intl/date_symbol_data_local.dart';

class UpdateReminder extends StatefulWidget {
  final Reminder reminder;

  const UpdateReminder({super.key, required this.reminder});

  @override
  State<UpdateReminder> createState() => _UpdateReminderState();
}

class _UpdateReminderState extends State<UpdateReminder> {
  @override
  void initState() {
    super.initState();
    initializeDateFormatting();
    _titleController = TextEditingController(text: widget.reminder.title);
  }

  final _formKey = GlobalKey<FormState>();
  late TextEditingController _titleController;
  final _dropdownController = TextEditingController();
  DateTime selectedDate = DateTime.now();

  Future<void> _selectDate(BuildContext context) async {
    final DateTime? picked = await showDatePicker(
      context: context,
      initialDate: widget.reminder.dueDate,
      firstDate: DateTime(2015, 8),
      lastDate: DateTime(2101),
      helpText: 'Select due date',
      builder: (context, child) {
        return Theme(
          data: ThemeData.dark().copyWith(
            primaryColor: const Color.fromARGB(232, 231, 105, 29),
            colorScheme: ColorScheme.fromSwatch(
              primarySwatch: Colors.orange,
              accentColor: const Color.fromARGB(232, 231, 105, 29),
              backgroundColor: Theme.of(context).colorScheme.background,
              cardColor: Theme.of(context).colorScheme.background,
              brightness: Theme.of(context).brightness,
            ),
          ),
          child: child!,
        );
      },
    );

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
        title: const Text('Update Reminder'),
        backgroundColor: const Color.fromARGB(232, 231, 105, 29),
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
                    const SizedBox(height: 20),
                    IconButton(
                      onPressed: () => _selectDate(context),
                      icon: const Icon(Icons.calendar_today),
                      tooltip: 'Select due date',
                    )
                  ],
                ),
                const SizedBox(height: 50),
                DropdownButtonFormField(
                  items: const [
                    DropdownMenuItem(
                      value: 'Ash',
                      child: Text('Ash'),
                    ),
                    DropdownMenuItem(
                      value: 'Sam',
                      child: Text('Sam'),
                    ),
                  ],
                  onChanged: (value) {
                    setState(() {
                      _dropdownController.text = value.toString();
                    });
                  },
                  value: _dropdownController.text.isEmpty
                      ? widget.reminder.assignee
                      : _dropdownController.text,
                ),
                const SizedBox(height: 200),
                Center(
                  child: ElevatedButton(
                    onPressed: () {
                      if (_formKey.currentState == null) {
                        return;
                      } else if (_formKey.currentState!.validate()) {
                        final due = DateTime(selectedDate.year,
                            selectedDate.month, selectedDate.day);

                        final reminder = Reminder(
                          id: widget.reminder.id,
                          title: _titleController.text,
                          due: due.millisecondsSinceEpoch ~/ 1000,
                          priority: 1000,
                          assignee: _dropdownController.text.isEmpty
                              ? null
                              : _dropdownController.text,
                        );
                        reminders.put(reminder).then((_) {
                          Navigator.pop(context);
                        });
                      }
                    },
                    style: ElevatedButton.styleFrom(
                      minimumSize: const Size(200, 60),
                      foregroundColor: const Color.fromARGB(232, 231, 105, 29),
                    ),
                    child: const Text('Submit'),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
