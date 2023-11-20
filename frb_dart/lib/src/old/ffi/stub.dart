import 'dart:async';

import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

import 'io.dart' if (dart.library.html) 'web.dart' show DartPostCObject;

export 'io.dart' if (dart.library.html) 'web.dart' show ExternalLibrary, DartApiDl;

int castInt(Object? value) => value as int;

abstract class FlutterRustBridgeWasmWireBase<T extends WasmModule> extends FlutterRustBridgeWireBase {
  Future<T> get init => throw UnimplementedError();

  FlutterRustBridgeWasmWireBase([FutureOr<T>? module]);
}

class JS {
  const JS([String? name]);
}

class _Anonymous {
  const _Anonymous();
}

const anonymous = _Anonymous();
