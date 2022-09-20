#![no_main]
use evtx::EvtxParser;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let data = std::io::Cursor::new(data);
    if let Ok(mut parser) = EvtxParser::from_read_seek(data) {
        for _ in parser.records() {}
    }
});
