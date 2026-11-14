#![allow(non_snake_case, non_camel_case_types)]
mod parser;

use std::ffi::{c_char, c_int, c_uint, c_void, CStr, CString};
use std::sync::atomic::{AtomicUsize, Ordering};

static ENGINE_ID: AtomicUsize = AtomicUsize::new(1);

#[link(name = "ibus-1.0")]
unsafe extern "C" {
    fn ibus_init();
    fn ibus_main();
    fn ibus_bus_new() -> *mut c_void;
    fn ibus_bus_get_connection(bus: *mut c_void) -> *mut c_void;
    fn ibus_bus_request_name(bus: *mut c_void, name: *const c_char, flags: c_uint) -> c_uint;
    fn ibus_factory_new(connection: *mut c_void) -> *mut c_void;
    fn ibus_engine_new(engine_name: *const c_char, object_path: *const c_char, connection: *mut c_void) -> *mut c_void;
    fn ibus_text_new_from_string(str: *const c_char) -> *mut c_void;
    fn ibus_engine_commit_text(engine: *mut c_void, text: *mut c_void);
    fn ibus_engine_update_preedit_text(engine: *mut c_void, text: *mut c_void, cursor_pos: c_uint, visible: c_int);
    fn ibus_engine_hide_preedit_text(engine: *mut c_void);
    fn ibus_lookup_table_new(page_size: c_uint, cursor_pos: c_uint, cursor_visible: c_int, round: c_int) -> *mut c_void;
    fn ibus_lookup_table_clear(table: *mut c_void);
    fn ibus_lookup_table_append_candidate(table: *mut c_void, text: *mut c_void);
    fn ibus_engine_update_lookup_table(engine: *mut c_void, table: *mut c_void, visible: c_int);
    fn ibus_engine_hide_lookup_table(engine: *mut c_void);
}

#[link(name = "gobject-2.0")]
unsafe extern "C" {
    fn g_signal_connect_data(
        instance: *mut c_void,
        detailed_signal: *const c_char,
        c_handler: unsafe extern "C" fn(),
        data: *mut c_void,
        destroy_data: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
        connect_flags: c_int,
    ) -> usize;
    fn g_object_ref_sink(object: *mut c_void) -> *mut c_void;
}

struct EngineState {
    buffer: String,
    lookup_table: *mut c_void,
}

impl EngineState {
    unsafe fn update(&self, engine: *mut c_void) {
        unsafe {
            if self.buffer.is_empty() {
                ibus_engine_hide_preedit_text(engine);
                ibus_engine_hide_lookup_table(engine);
                return;
            }
            let converted = parser::parse_avro(&self.buffer);
            let char_len = converted.chars().count() as c_uint;
            if let Ok(c_conv) = CString::new(converted) {
                let text = ibus_text_new_from_string(c_conv.as_ptr());
                ibus_engine_update_preedit_text(engine, text, char_len, 1);

                ibus_lookup_table_clear(self.lookup_table);
                let cand1 = ibus_text_new_from_string(c_conv.as_ptr());
                ibus_lookup_table_append_candidate(self.lookup_table, cand1);
                if let Ok(c_buf) = CString::new(self.buffer.as_str()) {
                    let cand2 = ibus_text_new_from_string(c_buf.as_ptr());
                    ibus_lookup_table_append_candidate(self.lookup_table, cand2);
                }
                ibus_engine_update_lookup_table(engine, self.lookup_table, 1);
            }
        }
    }

    unsafe fn commit(&mut self, engine: *mut c_void) {
        unsafe {
            if self.buffer.is_empty() {
                return;
            }
            let converted = parser::parse_avro(&self.buffer);
            if let Ok(c_conv) = CString::new(converted) {
                let text = ibus_text_new_from_string(c_conv.as_ptr());
                ibus_engine_commit_text(engine, text);
            }
            self.buffer.clear();
            ibus_engine_hide_preedit_text(engine);
            ibus_engine_hide_lookup_table(engine);
        }
    }
}

unsafe extern "C" fn on_process_key_event(
    engine: *mut c_void,
    keyval: c_uint,
    _keycode: c_uint,
    state: c_uint,
    user_data: *mut c_void,
) -> c_int {
    unsafe {
        if (state & (1 << 30)) != 0 || (state & ((1 << 2) | (1 << 3))) != 0 {
            return 0;
        }
        let eng_state = &mut *(user_data as *mut EngineState);

        if keyval == 0xff08 {
            if !eng_state.buffer.is_empty() {
                eng_state.buffer.pop();
                eng_state.update(engine);
                return 1;
            }
            return 0;
        }

        if keyval == 0x020 || keyval == 0xff0d || keyval == 0xff09 {
            if !eng_state.buffer.is_empty() {
                eng_state.commit(engine);
            }
            return 0;
        }

        if keyval == 0xff1b {
            if !eng_state.buffer.is_empty() {
                eng_state.buffer.clear();
                eng_state.update(engine);
                return 1;
            }
            return 0;
        }

        if (33..=126).contains(&keyval) {
            if let Some(ch) = char::from_u32(keyval) {
                eng_state.buffer.push(ch);
                eng_state.update(engine);
                return 1;
            }
        }
        0
    }
}

unsafe extern "C" fn on_focus_out(engine: *mut c_void, user_data: *mut c_void) {
    unsafe {
        let eng_state = &mut *(user_data as *mut EngineState);
        eng_state.commit(engine);
    }
}

unsafe extern "C" fn on_create_engine(
    _factory: *mut c_void,
    engine_name: *const c_char,
    conn: *mut c_void,
) -> *mut c_void {
    unsafe {
        let name_str = CStr::from_ptr(engine_name).to_str().unwrap_or("avro-rust");
        if name_str != "avro-rust" {
            return std::ptr::null_mut();
        }
        let id = ENGINE_ID.fetch_add(1, Ordering::SeqCst);
        let path = CString::new(format!("/org/freedesktop/IBus/Engine/{}", id)).unwrap();
        let engine = ibus_engine_new(engine_name, path.as_ptr(), conn);

        let table = g_object_ref_sink(ibus_lookup_table_new(5, 0, 1, 1));
        let state = Box::into_raw(Box::new(EngineState {
            buffer: String::new(),
            lookup_table: table,
        })) as *mut c_void;

        let sig_key = CString::new("process-key-event").unwrap();
        let cb_key: unsafe extern "C" fn(*mut c_void, c_uint, c_uint, c_uint, *mut c_void) -> c_int = on_process_key_event;
        g_signal_connect_data(
            engine,
            sig_key.as_ptr(),
            std::mem::transmute(cb_key),
            state,
            None,
            0,
        );

        let sig_focus = CString::new("focus-out").unwrap();
        let cb_focus: unsafe extern "C" fn(*mut c_void, *mut c_void) = on_focus_out;
        g_signal_connect_data(
            engine,
            sig_focus.as_ptr(),
            std::mem::transmute(cb_focus),
            state,
            None,
            0,
        );

        engine
    }
}

fn main() {
    unsafe {
        ibus_init();
        let bus = ibus_bus_new();
        let conn = ibus_bus_get_connection(bus);
        let factory = ibus_factory_new(conn);

        let sig_create = CString::new("create-engine").unwrap();
        let cb_create: unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_void) -> *mut c_void = on_create_engine;
        g_signal_connect_data(
            factory,
            sig_create.as_ptr(),
            std::mem::transmute(cb_create),
            conn,
            None,
            0,
        );

        let comp_name = CString::new("org.freedesktop.IBus.AvroRust").unwrap();
        ibus_bus_request_name(bus, comp_name.as_ptr(), 0);
        ibus_main();
    }
}

#[cfg(test)]
mod tests {
    use super::parser::parse_avro;

    #[test]
    fn test_bengali_avro() {
        assert_eq!(parse_avro("ami"), "আমি");
        assert_eq!(parse_avro("bangla"), "বাংলা");
        assert_eq!(parse_avro("ksh"), "ক্ষ");
        assert_eq!(parse_avro("gg"), "জ্ঞ");
        assert_eq!(parse_avro("ShT"), "ষ্ট");
        assert_eq!(parse_avro("q"), "ক");
        assert_eq!(parse_avro("wada"), "ওয়াদা");
        assert_eq!(parse_avro("swadhin"), "স্বাধিন");
        assert_eq!(parse_avro("."), "।");
        assert_eq!(parse_avro(".."), ".");
    }
}
