    // if false
    // {

    //     use xcb::*;

    //     let d = Connection::connect(None);
    //     let (conn, screen_num) = d.unwrap();
    //     let setup = conn.get_setup();
    //     let screen = setup.roots().nth(0).unwrap();
    // }

extern crate x11;

// TODO : delete me
// fn test_device_button_mapping() {
//     unsafe {
//         use self::x11::xlib;
//         use self::x11::xinput;
//         let disp: *mut xlib::Display = xlib::XOpenDisplay(std::ptr::null());
//         xinput::XListInputDevices(disp, );
//         xinput::XGetDeviceKeyMapping(disp);
//     }
// }

fn something() {
    ///////////////////////////////
    // https://docs.rs/x11/2.16.0/x11/xlib/enum._XDisplay.html
    // https://github.com/Kintaro/wtftw/blob/master/xlib/src/xlib_window_system.rs
    // https://stackoverflow.com/questions/45527720/how-can-i-make-a-window-override-redirect-with-glutin
    unsafe {
        use self::x11::xlib;
        use std::mem::uninitialized;
        use std;
        let disp: *mut xlib::Display = xlib::XOpenDisplay(std::ptr::null());
        // TODO : check return value?
        if disp.is_null() {
        }
        //let screen = xlib::XDefaultScreenOfDisplay(disp);
        //let root = xlib::XRootWindowOfScreen(screen);
        let mut event = xlib::XEvent { pad: [0; 24] };
        let mut attr = uninitialized();
        let win = 29360129; // or root
        //let mut atoms = uninitialized();
        //xlib::XGetAtomNames(disp, atoms
        xlib::XGetWindowAttributes(disp, win, &mut attr);
        println!("Attr: {:?}", attr);
        xlib::XChangeWindowAttributes(disp, win, xlib::CWEventMask, &mut xlib::XSetWindowAttributes {
            background_pixmap: 0,
            background_pixel: 0,
            border_pixmap: 0,
            border_pixel: 0,
            bit_gravity: 0,
            win_gravity: 0,
            backing_store: 0,
            backing_planes: 0,
            backing_pixel: 0,
            save_under: 0,
            event_mask: xlib::PropertyChangeMask,
            do_not_propagate_mask: 0,
            override_redirect: 0,
            colormap: 0,
            cursor: 0,
        });
        xlib::XGetWindowAttributes(disp, win, &mut attr);
        println!("Attr: {:?}", attr);
        loop {
            use std::os::raw::c_char;
            xlib::XNextEvent(disp, &mut event);
            println!("Event: {:?}", event);
            let mut name = uninitialized();
            xlib::XGetWMName(disp, win, &mut name);
            let y = ::std::ffi::CString::from_raw(name.value as *mut c_char);
            //let y = ::std::ffi::CStr::from_ptr(name.value as *const c_char);
            println!("Name: {:?}", y);
            // TODO : further filter the events based on atom -
            // ftp://www.x.org/pub/X11R7.7/doc/man/man3/XPropertyEvent.3.xhtml
        }
    }
}
