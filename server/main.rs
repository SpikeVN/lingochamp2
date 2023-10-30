/**
 * Copyright (c) 2022-2023 EpiX Team CBN.
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
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
pub mod api;
pub mod logging;
pub mod networking;

use crate::api::{Part, QuestionBank, Show, Status};

use std::net::SocketAddr;

struct TestPart {
    i: u32,
}

static mut Q_BANK: QuestionBank = QuestionBank::new();

impl Part for TestPart {
    fn on_update(&mut self) -> Status {
        self.i += 1;
        Status::RUNNING
    }
}

fn main() {
    logging::set_log_level(logging::Level::DEBUG);
    unsafe {
        Q_BANK.load_questions("assets/question_test.json");
    }
    println!("{}", unsafe {
        serde_json::to_string(Q_BANK.get_question(0)).unwrap()
    });
    networking::webservice::start_web_service(&SocketAddr::from(([127, 0, 0, 1], 3000)));
    let mut p1 = TestPart { i: 0 };
    let mut show = Show::new("Test Show", vec![&mut p1], 20);
    show.start();
}
