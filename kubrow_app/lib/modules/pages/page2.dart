import 'package:flutter/material.dart';

class Page2 extends StatelessWidget {
  const Page2({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return const Scaffold(
      body: Center(
        child: Text(
          'This is Page 2. Without any top bar.',
          style: TextStyle(fontSize: 24),
        ),
      ),
    );
  }
}
