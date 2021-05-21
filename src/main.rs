extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;
use gtk::{Application, ApplicationWindow, Adjustment, Entry, Grid, SpinButton};


use chrono::prelude::*;
//use log::{error, warn, info, debug, trace};
use log::{info, debug};
use std::convert::TryInto;

//https://en.wikipedia.org/wiki/Date_of_Easter#Algorithms

//fn calc_easter(year:i32) -> DateTime<chrono::Utc> {
fn calc_easter(year:i32) -> chrono::NaiveDate {
    info! ("entering calc_easter");
    let a = year % 19;
    debug!("a is  {}", a);
    let b = year / 100;
    debug!("b is  {}", b);
    let c = year % 100;
    debug!("c is  {}", c);
    let d = b / 4;
    debug!("d is  {}", d);
    let e = b % 4;
    debug!("e is  {}", e);
    let f = (b + 8) / 25;
    debug!("f is  {}", f);
    let g = (b -f + 1)/3;
    debug!("g is  {}", g);
    let h = (19 * a + b - d - g + 15) % 30;
    debug!("h is  {}", h);
    let i = c / 4;
    debug!("i is  {}", i);
    let k = c % 4;
    debug!("k is  {}", k);
    let l = (32 + 2*e + 2*i - h - k) % 7;
    debug!("l is  {}", l);
    let m = (a + 11*h + 22*l)/451;
    debug!("m is  {}", m);
    // type conversion to unsigned int
    let mon:u32 = ((h + l - 7*m + 114) / 31).try_into().unwrap();
    debug!("mon is  {}", mon);
    let day:u32 = (((h + l - 7*m + 114) % 31) + 1).try_into().unwrap();
    debug!("day is  {}", day);
    let easter = NaiveDate::from_ymd(year, mon, day);
    easter
}


fn main() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("What day is Easter?");
        window.set_default_size(350, 70);
        
        let adjustment = Adjustment::new(2021., 1500., 2500., 1., 10., 100.);
        let spinbutton = SpinButton::new(Some(&adjustment), 1.0, 0);
        let entry = Entry::new();
        
        let grid: Grid = Grid::new();
        entry.set_hexpand(true);
        grid.attach(&spinbutton, 0, 0, 1, 1);
        grid.attach(&entry, 0, 1, 1, 1);
        window.add(&grid);

        let entry_clone = entry.clone();
        spinbutton.connect_value_changed( move |spinbutton| {
            let year:i32 = spinbutton.get_value_as_int();
//            let rslt = calc_easter(year).to_rfc2822();
            let rslt = calc_easter(year).format("%A, %-d %B, %C%y").to_string();
            entry_clone.set_text(&rslt);
        });


        window.show_all();
    });

    application.run(&[]);
}

