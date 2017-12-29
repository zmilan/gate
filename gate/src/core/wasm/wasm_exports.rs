// Copyright 2017 Matthew D. Michelotti
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This module contains methods exported to the gate javascript code for WebAssembly.
//! DO NOT USE DIRECTLY!

use std::os::raw::c_int;

use ::input::{KeyEvent, KeyCode};
use super::APP_RUNNER;

#[no_mangle]
pub unsafe extern "C" fn gateWasmInit() {
    APP_RUNNER.r.borrow_mut().init();
}

#[no_mangle]
pub unsafe extern "C" fn gateWasmUpdateAndDraw(time_millis: f64) {
    APP_RUNNER.r.borrow_mut().update_and_draw(time_millis / 1000.0);
}

#[no_mangle]
pub unsafe extern "C" fn gateWasmKeyEvent(code: c_int, down: bool) {
    assert!(code >= 0 && code <= 255);
    let code = KeyCode::from_u8(code as u8).unwrap();
    let event = if down { KeyEvent::Pressed } else { KeyEvent::Released };
    APP_RUNNER.r.borrow_mut().input(event, code);
}
