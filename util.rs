pub type DataStream = @fn() -> Option<~[u8]>;

pub type DataStreamFactory = ~fn() -> DataStream;

