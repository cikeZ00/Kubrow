import 'package:flutter/material.dart';
import '../../ffi.dart';

class Page1 extends StatelessWidget {
  const Page1({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Page 1'),
      ),
      body: Center(
        child: ElevatedButton(
          child: const Text('Make Request'),
          onPressed: () {
            api.fetchData().then((_) {
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(content: Text('Request completed successfully')),
              );
            }).catchError((error) {
              ScaffoldMessenger.of(context).showSnackBar(
                SnackBar(content: Text('Request failed: $error')),
              );
            });
          },
        ),
      ),
    );
  }
}
