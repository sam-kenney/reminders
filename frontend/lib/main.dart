import 'package:flutter/material.dart';
import 'package:reminders/pages/home.dart';

const orange = Color.fromARGB(232, 231, 105, 29);

void main() {
  runApp(const App());
}

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Reminders',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: orange),
        useMaterial3: true,
      ),
      darkTheme: ThemeData.dark(),
      themeMode: ThemeMode.dark,
      home: const Home(title: 'Reminders'),
    );
  }
}
