import 'package:flutter/material.dart';
import 'ffi.dart';
import 'modules/home_page.dart';
import 'modules/pages.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Kubrow',
      theme: ThemeData(
        primarySwatch: Colors.lightBlue,
      ),
      initialRoute: '/',
      routes: {
        //'/': (context) => const MyHomePage(title: 'Kubrow'),
        '/page1': (context) => const Page1(),
        '/page2': (context) => const Page2(),
        '/page3': (context) => const Page3(),
      },
      home: const AppNavigationBar(),
    );
  }
}

class AppNavigationBar extends StatefulWidget {
  const AppNavigationBar({Key? key}) : super(key: key);

  @override
  _AppNavigationBarState createState() => _AppNavigationBarState();
}

class _AppNavigationBarState extends State<AppNavigationBar> {
  int _selectedIndex = 0;

  final List<Widget> _pages = const [
    MyHomePage(title: 'Kubrow'),
    Page1(),
    Page2(),
    Page3(),
  ];

  void _onItemTapped(int index) {
    setState(() {
      _selectedIndex = index;
    });
  }

  @override
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: _pages[_selectedIndex],
      bottomNavigationBar: BottomNavigationBar(
        items: const <BottomNavigationBarItem>[
          BottomNavigationBarItem(
            icon: Icon(Icons.home),
            label: 'Home',
          ),
          BottomNavigationBarItem(
            icon: Icon(Icons.map),
            label: 'Page 1',
          ),
          BottomNavigationBarItem(
            icon: Icon(Icons.local_library),
            label: 'Page 2',
          ),
          BottomNavigationBarItem(
            icon: Icon(Icons.settings),
            label: 'Page 3',
          ),
        ],
        currentIndex: _selectedIndex,
        onTap: _onItemTapped,
        backgroundColor: Colors.blue, // Set the background color
        //unselectedItemColor: Colors.black, // Set the unselected icon color
        selectedItemColor: Colors.white60,
        type: BottomNavigationBarType.fixed,
      ),
    );
  }
}
