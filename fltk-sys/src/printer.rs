/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Printer {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Printer_new() -> *mut Fl_Printer;
}
extern "C" {
    pub fn Fl_Printer_delete(self_: *mut Fl_Printer);
}
extern "C" {
    pub fn Fl_Printer_begin_job(
        self_: *mut Fl_Printer,
        pagecount: libc::c_int,
        frompage: *mut libc::c_int,
        topage: *mut libc::c_int,
        perr_message: *mut *mut libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Printer_begin_page(self_: *mut Fl_Printer) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Printer_printable_rect(
        self_: *mut Fl_Printer,
        w: *mut libc::c_int,
        h: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Printer_margins(
        self_: *mut Fl_Printer,
        left: *mut libc::c_int,
        top: *mut libc::c_int,
        right: *mut libc::c_int,
        bottom: *mut libc::c_int,
    );
}
extern "C" {
    pub fn Fl_Printer_origin(self_: *mut Fl_Printer, x: *mut libc::c_int, y: *mut libc::c_int);
}
extern "C" {
    pub fn Fl_Printer_set_origin(self_: *mut Fl_Printer, x: libc::c_int, y: libc::c_int);
}
extern "C" {
    pub fn Fl_Printer_scale(self_: *mut Fl_Printer, scale_x: f32, scale_y: f32);
}
extern "C" {
    pub fn Fl_Printer_rotate(self_: *mut Fl_Printer, angle: f32);
}
extern "C" {
    pub fn Fl_Printer_translate(self_: *mut Fl_Printer, x: libc::c_int, y: libc::c_int);
}
extern "C" {
    pub fn Fl_Printer_untranslate(self_: *mut Fl_Printer);
}
extern "C" {
    pub fn Fl_Printer_end_page(self_: *mut Fl_Printer) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Printer_end_job(self_: *mut Fl_Printer);
}
extern "C" {
    pub fn Fl_Printer_set_current(self_: *mut Fl_Printer);
}
extern "C" {
    pub fn Fl_Printer_is_current(self_: *mut Fl_Printer) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Printer_print_widget(
        self_: *mut Fl_Printer,
        widget: *mut libc::c_void,
        delta_x: libc::c_int,
        delta_y: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_Printer_print_window(
        self_: *mut Fl_Printer,
        win: *mut libc::c_void,
        x_offset: libc::c_int,
        y_offset: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_Printer_set_dialog_title(msg: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_printer(msg: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_range(msg: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_copies(msg: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_all(msg: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_pages(msg: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_from(msg: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_to(msg: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_properties(msg: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_copyNo(msg: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_print_button(msg: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_cancel_button(msg: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_dialog_print_to_file(msg: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_property_title(msg: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_property_pagesize(msg: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_property_mode(msg: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_property_use(msg: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_property_save(msg: *const libc::c_char);
}
extern "C" {
    pub fn Fl_Printer_set_property_cancel(msg: *const libc::c_char);
}
