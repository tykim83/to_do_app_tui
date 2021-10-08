use std::cell::RefCell;
use std::rc::Rc;

use cursive::{Cursive, traits::{Boxable, Nameable}, view::Margins, views::{Button, Checkbox, Dialog, DummyView, EditView, LinearLayout, ListChild, ListView, PaddedView}};

fn main() {
    // let done = Rc::new(vec![String::from("Something")]);
    // Rc::clone()
    // let todo = RefCell::new(vec![String::from("Else")]);

    let mut siv = cursive::default();

    let todo_view = Dialog::around(ListView::new().with_name("todo"))
        .title("ToDo")
        .min_height(10)
        .min_width(25);

    let done_view = Dialog::around(ListView::new().with_name("done"))
        .title("Done")
        .min_height(10)
        .min_width(25);

    let edit_view = EditView::new()
        .on_submit(add_todo)
        .with_name("input")
        .fixed_width(15);

    let add_view = PaddedView::new(
        Margins {
            left: 1,
            right: 1,
            top: 1,
            bottom: 1,
        },
        LinearLayout::horizontal()
            .child(edit_view)
            .child(DummyView)
            .child(Button::new("Add", |s| {
                let text = s
                    .call_on_name("input", |c: &mut EditView| c.get_content())
                    .unwrap();
                add_todo(s, text.as_str());
            }))
            .max_width(25),
    );

    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(LinearLayout::horizontal().child(todo_view).child(done_view))
                .child(add_view)
                .min_height(15)
                .min_width(50),
        )
        .padding(Margins::lrtb(1, 1, 1, 1))
        .button("Clear", |s| {
            s.call_on_name("done", |c: &mut ListView| {
                c.clear()
            });
        })
        .button("Quit", |s| s.quit())
        .title("ToDo App"),
    );

    siv.run();
}

fn add_todo(s: &mut Cursive, text: &str) {
    s.call_on_name("todo", |view: &mut ListView| {
        view.add_child(text, Checkbox::new().on_change(toogle_todo))
    });
    s.call_on_name("input", |view: &mut EditView| {
        view.set_content("");
    });
}

fn toogle_todo(s: &mut Cursive, checked: bool) {
    let label = match checked {
        true => get_label(s, "todo"),
        false => get_label(s, "done"),
    };

    match label {
        Some((label, index)) if checked => {
            done_add_child(s, label.as_str());
            todo_remove_child(s, index);
        }
        Some((label, index)) if !checked => {
            todo_add_child(s, label.as_str());
            done_remove_child(s, index);
        }
        Some(_) => (), 
        None => (),
    }
}

fn done_add_child(s: &mut Cursive, label: &str) {
    s.call_on_name("done", |view: &mut ListView| {
        view.add_child(label, Checkbox::new().on_change(toogle_todo).checked())
    });
}

fn todo_add_child(s: &mut Cursive, label: &str) {
    s.call_on_name("todo", |view: &mut ListView| {
        view.add_child(label, Checkbox::new().on_change(toogle_todo))
    });
}

fn done_remove_child(s: &mut Cursive, index: usize) {
    s.call_on_name("done", |c: &mut ListView| c.remove_child(index));
}

fn todo_remove_child(s: &mut Cursive, index: usize) {
    s.call_on_name("todo", |c: &mut ListView| c.remove_child(index));
}

fn get_label(s: &mut Cursive, name: &str) -> Option<(String, usize)> {
    s.call_on_name(name, |c: &mut ListView| {
        let index = c.focus();
        let child = c.get_row(index);
        match child {
            ListChild::Row(label, _) => (label.clone(), index),
            ListChild::Delimiter => todo!(),
        }
    })
}

// Cursive layout -> Ok
// Cursive actions -> need more research
// checkbox -> on_change -> get listview current selected

// Rc RefCell

//     let to_dos = RefCell::new(ToDos::new());

// Load Todo
// Load Done
// Add Todo -> View and Vec
// Toggle ToDo/Done -> View and Vec
// Add and Align clear and quit button
// Clear - Remove Done
// Refactor Code
// Load from file
// Quit - Save to file

// ToDo            | Done
// Learn Rust [ ]  | Eat [x]
//                 |
//                 |

// Add _________   | <Clear> <Quit>
