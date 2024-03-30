/*
 * Copyright (C) 2015 drrb
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

// Create a library, not an executable binary
#![crate_type = "dylib"]
#![feature(slice_ptr_len)]
#![feature(slice_ptr_get)]
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_float, c_int};
use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};
use std::str;
use crate::simpleAoi::{Entity, Aoi, run};

/// Example of passing and returning a value
/// The string argument and return types are native C strings (pointers to arrays of c_chars).
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn rust_print(name: *const c_char) -> *const c_char {
    let name = to_string(name);
    // Convert the Rust string back to a C string so that we can return it
    to_ptr(format!("Rust print: {}!", name))
}

/// Convert a native string to a Rust string
fn to_string(pointer: *const c_char) -> String {
    let slice = unsafe { CStr::from_ptr(pointer).to_bytes() };
    str::from_utf8(slice).unwrap().to_string()
}

/// Convert a Rust string to a native string
fn to_ptr(string: String) -> *const c_char {
    let cs = CString::new(string.as_bytes()).unwrap();
    let ptr = cs.as_ptr();
    // Tell Rust not to clean up the string while we still have a pointer to it.
    // Otherwise, we'll get a segfault.
    mem::forget(cs);
    ptr
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn rust_print2(clen: c_int, array: *const c_float) -> *const c_int {
    let slice = slice_from_raw_parts(array, clen as usize);
    let list = unsafe { &*slice };
    let len = list.len();
    println!("{:?}", list);
    let ptr = len as *const c_int;
    mem::forget(slice);
    ptr
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn rust_print3(clen: c_int, array: *mut c_float) -> *const c_int {
    let len = clen * 2;
    let list = slice_from_raw_parts_mut::<f32>(array, len as usize);
    unsafe {
        for i in 0..len {
            let v = i as f32 + 1f32;
            (*list)[i as usize] = v;
        }
    };
    mem::forget(list);
    println!("rust {:?}", len);
    let ptr = len as *const c_int;
    ptr
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn rust_print4(clen: c_int) -> *mut c_float {
    let len = clen;
    let mut vec = Vec::<f32>::with_capacity(len as usize);
    for i in 0..len {
        let v = i as f32 + 1f32;
        vec.push(i as f32);
    }
    vec.as_mut_ptr()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn aoi(clen: c_int, array: *const c_float) -> *const c_int {
    let slice = slice_from_raw_parts(array, clen as usize);
    let list = unsafe { &*slice };
    // println!("{}",list[0]);
    // let ptr = len as *const c_int;
    // mem::forget(slice);
    // ptr
    let input = get_entitys(list);
    let mut r = run(input);
    let ptr = r.len() as *const c_int;
    mem::forget(slice);
    ptr
}

fn get_entitys(list: &[f32]) -> Vec<Entity> {
    let len = list.len() as i32;
    let num = len / 3;
    let mut vec = Vec::with_capacity(num as usize);
    for i in 0..num {
        let index = i * 3;
        let x = list[(index + 0) as usize];
        let y = list[(index + 1) as usize];
        let r = list[(index + 2) as usize];
        let e = Entity::new(x, y, r);
        vec.push(e);
    }
    vec
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn test_obj(obj:*mut Entity) -> *const c_char {
    println!("Rust print: {}!", unsafe { obj.as_ref().unwrap()});
    let e=unsafe{
        obj.replace(Entity::new(10 as f32, 20 as f32, 30 as f32));
        &*obj
    };
    to_ptr(format!("Rust return: {}!", e))
}
