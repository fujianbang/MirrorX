import 'dart:developer';

import 'package:mirrorx/env/langs/tr.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mirrorx/global_state/global_state_cubit.dart';

class DeviceIdField extends StatelessWidget {
  const DeviceIdField({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 110,
      width: 360,
      decoration: const BoxDecoration(
        border: Border(left: BorderSide(color: Colors.yellow, width: 4)),
      ),
      child: Padding(
        padding: const EdgeInsets.only(left: 12.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text(
                  Tr.of(context).connectPageDeviceIDTitle,
                  style: const TextStyle(fontSize: 27),
                ),
                BlocBuilder<GlobalStateCubit, GlobalState>(
                  builder: (context, state) => IconButton(
                    onPressed: () {
                      Clipboard.setData(ClipboardData(text: state.deviceID))
                          .then(
                        (_) => ScaffoldMessenger.of(context).showSnackBar(SnackBar(
                            content: Text(Tr.of(context)
                                .connectPageDeviceIDButtonCopySnackbarContent))),
                      );
                    },
                    icon: const Icon(Icons.copy),
                    splashRadius: 20,
                    hoverColor: Colors.yellow,
                    tooltip:
                        Tr.of(context).connectPageDeviceIDButtonCopyTooltip,
                  ),
                ),
              ],
            ),
            Expanded(
              child: FutureBuilder(
                future: context.read<GlobalStateCubit>().fetchDeviceID(),
                builder: (context, snapshot) {
                  switch (snapshot.connectionState) {
                    case ConnectionState.none:
                    case ConnectionState.waiting:
                    case ConnectionState.active:
                      return const Center(child: CircularProgressIndicator());
                    case ConnectionState.done:
                      if (snapshot.hasError) {
                        log("Error: ${snapshot.error}");
                        return const Icon(Icons.report, color: Colors.red);
                      } else {
                        return Text(
                          snapshot.data.toString(),
                          style: const TextStyle(fontSize: 45),
                        );
                      }
                  }
                },
              ),
            )
          ],
        ),
      ),
    );
  }
}
