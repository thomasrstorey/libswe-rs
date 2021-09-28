/*  libswe-rs | Rust bindings for libswe, the Swiss Ephemeris C library.
 *  Copyright (c) 2021 Thomas R Storey. All rights reserved.

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as
    published by the Free Software Foundation, either version 3 of the
    License, or (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

extern crate libswe;

use chrono::{TimeZone, Utc};
use libswe::core::{Body, Flag};

fn main() {
    libswe::core::set_ephe_path(Option::None);
    let julian_day_ut = libswe::core::julday(Utc.ymd(1991, 10, 13).and_hms(20, 0, 0));
    println!("Planet\tlon\tlat\tdist");
    for body in Body::Sun..Body::Chiron {
        if body == Body::Earth {
            continue;
        }
        let flag_set = [Flag::HighPrecSpeed];
        let calc_result = libswe::core::calc_ut(julian_day_ut, body, &flag_set);
        if calc_result.code < 0 {
            eprintln!("Error: {}", calc_result.error);
        } else {
            let name = libswe::core::get_planet_name(body);

            println!(
                "{}\t{}\t{}\t{}",
                name, calc_result.lng, calc_result.lat, calc_result.dist,
            );
        }
    }
    libswe::core::close();
}
