#![feature(test)]
extern crate test;
extern crate tera;
#[macro_use]
extern crate serde_derive;

use tera::{Tera, Template, Context, escape_html};

#[derive(Serialize)]
struct DataWrapper {
    i: usize,
    v: String,
}

impl DataWrapper {
    fn new(i: usize) ->  DataWrapper {
        DataWrapper { i, v: "Meta
Before we get to the details, two important notes about the ownership system.

Rust has a focus on safety and speed. It accomplishes these goals through many ‘zero-cost abstractions’, which means that in Rust, abstractions cost as little as possible in order to make them work. The ownership system is a prime example of a zero cost abstraction. All of the analysis we’ll talk about in this guide is done at compile time. You do not pay any run-time cost for any of these features.

However, this system does have a certain cost: learning curve. Many new users to Rust experience something we like to call ‘fighting with the borrow checker’, where the Rust compiler refuses to compile a program that the author thinks is valid. This often happens because the programmer’s mental model of how ownership should work doesn’t match the actual rules that Rust implements. You probably will experience similar things at first. There is good news, however: more experienced Rust developers report that once they work with the rules of the ownership system for a period of time, they fight the borrow checker less and less.

With that in mind, let’s learn about borrowing.".into() }
    }
}

#[derive(Serialize)]
struct BigObject {
    field_a: DataWrapper,
    field_b: DataWrapper,
    field_c: DataWrapper,
    field_d: DataWrapper,    
    field_e: DataWrapper,
    field_f: DataWrapper,    
}

impl BigObject {
    fn new(i: usize) -> BigObject {
        BigObject {
            field_a: DataWrapper::new(i),
            field_b: DataWrapper::new(i),
            field_c: DataWrapper::new(i),
            field_d: DataWrapper::new(i),    
            field_e: DataWrapper::new(i),
            field_f: DataWrapper::new(i),    
        }
    }
}


#[bench]
fn bench_big_loop_big_object(b: &mut test::Bencher) {
    const NUM_OBJECTS: usize = 100;
    let mut objects = Vec::with_capacity(NUM_OBJECTS);
    for i in 0..NUM_OBJECTS {
        objects.push(BigObject::new(i));
    }

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("big_loop.html", "
{%- for object in objects -%}
{{ object.field_a.i }}
{%- if object.field_a.i > 2 -%}
{%- break -%}
{%- endif -%}
{%- endfor -%}
"),
    ]).unwrap();
    let mut context = Context::new();
    context.add("objects", &objects);
    let rendering = tera.render("big_loop.html", &context).expect("Good render");
    assert_eq!(&rendering[..], "0123");
    b.iter(|| tera.render("big_loop.html", &context));
}

#[bench]
fn bench_macro_big_object(b: &mut test::Bencher) {
    let big_object = BigObject::new(1);
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("big_loop.html", "
{%- import \"macros.html\" as macros -%}
{%- for i in iterations -%}{{ macros::get_first(bo=big_object) }}{% endfor %}"),
        ("macros.html", "{%- macro get_first(bo) -%}{{ bo.field_a.i }}{% endmacro get_first %}"),
    ]).unwrap();
    let mut context = Context::new();
    context.add("big_object", &big_object);
    context.add("iterations", &(0..500).collect::<Vec<usize>>());
    let rendering = tera.render("big_loop.html", &context).expect("Good render");
    assert_eq!(rendering.len(), 500);
    assert_eq!(rendering.chars().next().expect("Char"), '1');
    b.iter(|| tera.render("big_loop.html", &context));
}

