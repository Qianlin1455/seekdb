// Copyright (c) 2026 OceanBase.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cxx::bridge(namespace = "oceanbase::in_process")]
mod ffi {
    unsafe extern "C++" {
        include!("in_process_server.h");

        #[allow(dead_code)]
        fn run_server(directory: &str, parameters: &[String], report_running: fn()) -> i32;
    }
}

#[cfg(not(test))]
pub(crate) use ffi::run_server;

#[cfg(test)]
pub(crate) use tests::run_server;

#[cfg(test)]
pub(crate) mod tests {
    use std::io::Write;
    use std::os::unix::net::UnixListener;
    use std::sync::Mutex;

    pub(crate) static PARAMETERS: Mutex<Vec<String>> = Mutex::new(Vec::new());

    pub(crate) fn run_server(_directory: &str, parameters: &[String], report_running: fn()) -> i32 {
        *PARAMETERS.lock().unwrap() = parameters.to_vec();
        let Ok(listener) = UnixListener::bind("run/sql.sock") else {
            return -1;
        };
        report_running();
        for mut stream in listener.incoming().flatten() {
            let _ = stream.write_all(&[74, 0, 0, 0, 10]);
        }
        0
    }
}
