#![allow(unused_variables)]//允许未使用的变量
#![allow(dead_code)]//允许未使用的代码
#![allow(unused_must_use)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis_macro_driver;

#[macro_use]
pub mod util;
pub mod domain;
pub mod dao;
pub mod controller;
pub mod service;
pub mod config;
pub mod middleware;
pub mod view;
pub mod base;