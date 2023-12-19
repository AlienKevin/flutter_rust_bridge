// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.2.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<void> rustAutoOpaqueArgOwnTwinSse(
        {required RwLockNonCloneSimpleTwinSse arg,
        required int expect,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueArgOwnTwinSse(arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueArgBorrowTwinSse(
        {required RwLockNonCloneSimpleTwinSse arg,
        required int expect,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueArgBorrowTwinSse(arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueArgMutBorrowTwinSse(
        {required RwLockNonCloneSimpleTwinSse arg,
        required int expect,
        required int adder,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueArgMutBorrowTwinSse(
        arg: arg, expect: expect, adder: adder, hint: hint);

Future<RwLockNonCloneSimpleTwinSse> rustAutoOpaqueReturnOwnTwinSse(
        {required int initial, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueReturnOwnTwinSse(initial: initial, hint: hint);

Future<RwLockNonCloneSimpleTwinSse> rustAutoOpaqueArgOwnAndReturnOwnTwinSse(
        {required RwLockNonCloneSimpleTwinSse arg, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueArgOwnAndReturnOwnTwinSse(arg: arg, hint: hint);

Future<void> rustAutoOpaqueTwoArgsTwinSse(
        {required RwLockNonCloneSimpleTwinSse a,
        required RwLockNonCloneSimpleTwinSse b,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTwoArgsTwinSse(a: a, b: b, hint: hint);

Future<void> rustAutoOpaqueNormalAndOpaqueArgTwinSse(
        {required RwLockNonCloneSimpleTwinSse a,
        required String b,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueNormalAndOpaqueArgTwinSse(a: a, b: b, hint: hint);

/// "+" inside the type signature
Future<void> rustAutoOpaquePlusSignArgTwinSse(
        {required RwLockBoxMyTraitTwinSse arg, dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaquePlusSignArgTwinSse(arg: arg, hint: hint);

Future<RwLockBoxMyTraitTwinSse> rustAutoOpaquePlusSignReturnTwinSse(
        {dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaquePlusSignReturnTwinSse(hint: hint);

Future<void> rustAutoOpaqueCallableArgTwinSse(
        {required RwLockBoxFnStringString arg, dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueCallableArgTwinSse(arg: arg, hint: hint);

Future<RwLockBoxFnStringString> rustAutoOpaqueCallableReturnTwinSse(
        {dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueCallableReturnTwinSse(hint: hint);

Future<void> rustAutoOpaqueTraitObjectArgOwnTwinSse(
        {required RwLockBoxHelloTraitTwinSse arg,
        required String expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTraitObjectArgOwnTwinSse(
        arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueTraitObjectArgBorrowTwinSse(
        {required RwLockBoxHelloTraitTwinSse arg,
        required String expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTraitObjectArgBorrowTwinSse(
        arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueTraitObjectArgMutBorrowTwinSse(
        {required RwLockBoxHelloTraitTwinSse arg,
        required String expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTraitObjectArgMutBorrowTwinSse(
        arg: arg, expect: expect, hint: hint);

Future<RwLockBoxHelloTraitTwinSse> rustAutoOpaqueTraitObjectReturnOwnOneTwinSse(
        {dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueTraitObjectReturnOwnOneTwinSse(hint: hint);

Future<RwLockBoxHelloTraitTwinSse> rustAutoOpaqueTraitObjectReturnOwnTwoTwinSse(
        {dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueTraitObjectReturnOwnTwoTwinSse(hint: hint);

Future<void> rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinSse(
        {required RwLockStructWithGoodAndOpaqueFieldTwinSse arg,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinSse(
            arg: arg, hint: hint);

Future<void> rustAutoOpaqueStructWithGoodAndOpaqueFieldArgBorrowTwinSse(
        {required RwLockStructWithGoodAndOpaqueFieldTwinSse arg,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueStructWithGoodAndOpaqueFieldArgBorrowTwinSse(
            arg: arg, hint: hint);

Future<void> rustAutoOpaqueStructWithGoodAndOpaqueFieldArgMutBorrowTwinSse(
        {required RwLockStructWithGoodAndOpaqueFieldTwinSse arg,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueStructWithGoodAndOpaqueFieldArgMutBorrowTwinSse(
            arg: arg, hint: hint);

Future<RwLockStructWithGoodAndOpaqueFieldTwinSse>
    rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinSse(
            {dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinSse(
                hint: hint);

// Rust type: flutter_rust_bridge::RustOpaque<std::sync::RwLock<Box<dyn Fn (String) -> String + Send + Sync + UnwindSafe + RefUnwindSafe>>>
@sealed
class RwLockBoxFnStringString extends RustOpaque {
  RwLockBoxFnStringString.dcoDecode(dynamic wire)
      : super.dcoDecode(wire, _kStaticData);

  RwLockBoxFnStringString.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_RwLockBoxFnStringString,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_RwLockBoxFnStringString,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_RwLockBoxFnStringStringPtr,
  );
}

// Rust type: flutter_rust_bridge::RustOpaque<std::sync::RwLock<Box<dyn HelloTraitTwinSse>>>
@sealed
class RwLockBoxHelloTraitTwinSse extends RustOpaque {
  RwLockBoxHelloTraitTwinSse.dcoDecode(dynamic wire)
      : super.dcoDecode(wire, _kStaticData);

  RwLockBoxHelloTraitTwinSse.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_RwLockBoxHelloTraitTwinSse,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_RwLockBoxHelloTraitTwinSse,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_RwLockBoxHelloTraitTwinSsePtr,
  );
}

// Rust type: flutter_rust_bridge::RustOpaque<std::sync::RwLock<Box<dyn MyTraitTwinSse + Send + Sync>>>
@sealed
class RwLockBoxMyTraitTwinSse extends RustOpaque {
  RwLockBoxMyTraitTwinSse.dcoDecode(dynamic wire)
      : super.dcoDecode(wire, _kStaticData);

  RwLockBoxMyTraitTwinSse.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_RwLockBoxMyTraitTwinSse,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_RwLockBoxMyTraitTwinSse,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_RwLockBoxMyTraitTwinSsePtr,
  );
}

// Rust type: flutter_rust_bridge::RustOpaque<std::sync::RwLock<NonCloneSimpleTwinSse>>
@sealed
class RwLockNonCloneSimpleTwinSse extends RustOpaque {
  RwLockNonCloneSimpleTwinSse.dcoDecode(dynamic wire)
      : super.dcoDecode(wire, _kStaticData);

  RwLockNonCloneSimpleTwinSse.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_RwLockNonCloneSimpleTwinSse,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_RwLockNonCloneSimpleTwinSse,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_RwLockNonCloneSimpleTwinSsePtr,
  );

  Future<void> instanceMethodArgBorrowTwinSse({dynamic hint}) =>
      RustLib.instance.api.nonCloneSimpleTwinSseInstanceMethodArgBorrowTwinSse(
        that: this,
      );

  Future<void> instanceMethodArgMutBorrowTwinSse({dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinSseInstanceMethodArgMutBorrowTwinSse(
        that: this,
      );

  Future<void> instanceMethodArgOwnTwinSse({dynamic hint}) =>
      RustLib.instance.api.nonCloneSimpleTwinSseInstanceMethodArgOwnTwinSse(
        that: this,
      );

  Future<RwLockNonCloneSimpleTwinSse> instanceMethodReturnOwnTwinSse(
          {dynamic hint}) =>
      RustLib.instance.api.nonCloneSimpleTwinSseInstanceMethodReturnOwnTwinSse(
        that: this,
      );

  /// named constructor
  static Future<RwLockNonCloneSimpleTwinSse> newCustomNameTwinSse(
          {dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinSseNewCustomNameTwinSse(hint: hint);

  /// unnamed constructor
  static Future<RwLockNonCloneSimpleTwinSse> newTwinSse({dynamic hint}) =>
      RustLib.instance.api.nonCloneSimpleTwinSseNewTwinSse(hint: hint);

  static Future<void> staticMethodArgBorrowTwinSse(
          {required RwLockNonCloneSimpleTwinSse arg, dynamic hint}) =>
      RustLib.instance.api.nonCloneSimpleTwinSseStaticMethodArgBorrowTwinSse(
          arg: arg, hint: hint);

  static Future<void> staticMethodArgMutBorrowTwinSse(
          {required RwLockNonCloneSimpleTwinSse arg, dynamic hint}) =>
      RustLib.instance.api.nonCloneSimpleTwinSseStaticMethodArgMutBorrowTwinSse(
          arg: arg, hint: hint);

  static Future<void> staticMethodArgOwnTwinSse(
          {required RwLockNonCloneSimpleTwinSse arg, dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinSseStaticMethodArgOwnTwinSse(arg: arg, hint: hint);

  static Future<RwLockNonCloneSimpleTwinSse> staticMethodReturnOwnTwinSse(
          {dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinSseStaticMethodReturnOwnTwinSse(hint: hint);
}

// Rust type: flutter_rust_bridge::RustOpaque<std::sync::RwLock<StructWithGoodAndOpaqueFieldTwinSse>>
@sealed
class RwLockStructWithGoodAndOpaqueFieldTwinSse extends RustOpaque {
  RwLockStructWithGoodAndOpaqueFieldTwinSse.dcoDecode(dynamic wire)
      : super.dcoDecode(wire, _kStaticData);

  RwLockStructWithGoodAndOpaqueFieldTwinSse.sseDecode(
      int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_RwLockStructWithGoodAndOpaqueFieldTwinSse,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_RwLockStructWithGoodAndOpaqueFieldTwinSse,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_RwLockStructWithGoodAndOpaqueFieldTwinSsePtr,
  );
}
