@0x9fa3b5c5f74e8f9e;

struct UIComponentTree {
  uiComponents @0 :List(UIComponentTree);
}

struct AddCommand {
  a @0 :Int64;
  b @1 :Int64;
}

struct Command {
  union {
    ping         @0 :Void;
    echo         @1 :Text;
    add          @2 :AddCommand;
    time         @3 :Void;
    getInitialUI @4 :Void;
  }
}

struct Response {
  union {
    pong   @0 :Text;
    number @1 :Int64;
    text   @2 :Text;
    uiInit @3 :UIComponentTree;
  }
}
