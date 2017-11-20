@0xb50f0b310e788f3d;

struct TimeStamp {
    secs @0: UInt64;
    subnanos @1: UInt32;
}

struct Vector3 {
    x @0: Float64;
    y @1: Float64;
    z @2: Float64;
}

struct Quaternion {
    x @0: Float64;
    y @1: Float64;
    z @2: Float64;
    w @3: Float64;
}

struct Transform {
    translation @0: Vector3;
    rotation @1: Quaternion;
}

struct Point {
    x @0: Float64;
    y @1: Float64;
    z @2: Float64;
}

struct LogMsg {
    severity @0 :Severity;
    timestamp @1 :TimeStamp;
    name @2 :Text;
    msg @3 :Text;



    enum Severity {
       trace @0;
       debug @1;
       info @2;
       warn @3;
       error @4;
       fatal @5;
    }
}

struct Topic {
    name @0 :Text;
    msgtype @1 :UInt64;
    msg @2 :AnyPointer;
}