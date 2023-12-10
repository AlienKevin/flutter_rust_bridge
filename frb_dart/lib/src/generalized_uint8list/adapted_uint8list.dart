import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/generalized_uint8list/generalized_uint8list.dart';

/// {@macro flutter_rust_bridge.internal}
// The name is just aligned to the RustVecU8.
typedef AdaptedUint8ListRaw = ({Uint8List ptr, int length});

/// Thin adapter to let [Uint8List] match interface of [BaseGeneralizedUint8List]
class AdaptedUint8List
    implements BaseGeneralizedUint8List<AdaptedUint8ListRaw> {
  Uint8List _inner;

  /// {@macro flutter_rust_bridge.internal}
  AdaptedUint8List(int length, GeneralizedFrbRustBinding binding)
      : _inner = Uint8List(length);

  @override
  int get length => _inner.length;

  @override
  void operator []=(int index, int value) => _inner[index] = value;

  @override
  void dispose() {}

  @override
  AdaptedUint8ListRaw intoRaw() => (ptr: _inner, length: _inner.length);

  @override
  void resize(int newLen) {
    final old = _inner;
    _inner = Uint8List(newLen);
    _inner.setRange(0, old.length, old);
  }

  @override
  void setRange(int start, int end, Uint8List data) =>
      _inner.setRange(start, end, data);
}
