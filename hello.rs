use ::libc;
use std::{ptr, slice};
use std::ffi::CStr;
use std::str::from_utf8;

/*
This needs to be compiled into a dynamic library, and needs the libc crate. Place the following in Cargo.toml
[dependencies]
libc = "0.2"
[lib]
name = "foo"
crate-type = ["dylib"]

Also, when building, make sure the tcl86 library is in the path. For Windows, I just copy tcl86.lib from the Tcl
distribution to the root of my project (where Cargo.toml is). I suppose it should be something similar for macOS and Linux
*/

pub type Tcl_CmdProc = extern "C" fn (cd:*mut libc::c_void, interp:*mut libc::c_void,
                                      argc: ::libc::c_int, argv: *mut *const ::libc::c_char) -> ::libc::c_int;

#[link(name = "tcl86")]
extern {
     pub fn Tcl_EvalEx(interp: *mut libc::c_void, script: *const ::libc::c_char,
                      numBytes: ::libc::c_int, flags: ::libc::c_int) -> ::libc::c_int;
    pub fn Tcl_CreateCommand(interp:*mut libc::c_void, cmdName: *const ::libc::c_char,
                             _proc: Tcl_CmdProc, cd:*mut libc::c_void, dp:*mut libc::c_void) -> *mut libc::c_void;
}

/*
If my dynamic library is called foo.dll then Tcl expects the entry function name to be: Foo_Init
This function takes care of registering my extension with the Tcl interpreter
*/
#[allow(non_snake_case)]
#[no_mangle]
pub extern fn Foo_Init(interp: *mut libc::c_void) -> ::libc::c_int {
    /*
    This creates a new Tcl command, which can be invoked
    mycmd str-0 str-1 ... str-n
    When this command is invoked from a Tcl script, it will call the myCommand function defined below
    It will pass all the trailing whitespace-separated strings as an array
    */
    unsafe { Tcl_CreateCommand(interp, "mycmd\0".as_ptr() as *const i8,
                               myCommand, ptr::null_mut() , ptr::null_mut()); }
    
    0 // This tells Tcl everything ended OK
}

/*
Function that will get called by the Tcl interpreter whenever mycmd is invoked
*/
extern "C" fn myCommand(_:*mut libc::c_void, interp: *mut libc::c_void,
                        argc: ::libc::c_int, argv: *mut *const ::libc::c_char) -> ::libc::c_int {
    let args = unsafe { slice::from_raw_parts(argv, argc as usize) };
    if argc > 1 {
        let cmd = unsafe { from_utf8(CStr::from_ptr(args[1]).to_bytes()).unwrap() };
        /*
        Implements a "greet" sub-command to place a greeting in a given label widget
        Tcl should send an array of 3 strings:
        0: always the command name; in this case "mycmd"
        1: the widget name
        3: who I will greet
        So, if in my Tcl code I write:
        mycmd greet .greet World
        then this will request the Tcl interpreter to execute:
        .greet configure -text {Hello, World!}
        */
        
        if cmd == "greet" && argc > 3 {
            let widget  = unsafe { from_utf8(CStr::from_ptr(args[2]).to_bytes()).unwrap() };
            let message = unsafe { from_utf8(CStr::from_ptr(args[3]).to_bytes()).unwrap() };
            let script = format!("{} configure -text {{Hello, {}!}}", widget, message);
            unsafe { Tcl_EvalEx(interp, script.as_ptr() as *const i8, script.len() as i32, 0); }
        }
    }
    
    0 // This tells Tcl everything ended OK
}
