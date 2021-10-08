use std::cell::RefCell;
use std::rc::Rc;

use cursive::{Cursive, traits::{Boxable, Nameable}, view::Margins, views::{Button, TextView,Checkbox, Dialog, DummyView, EditView, LinearLayout, ListView, PaddedView}};

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
            .child(Button::new("Add", |s| s.quit()))
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
        .button("Clear", |s| s.quit())
        .button("Quit", show_popup)
        .title("ToDo App"),
    );

    siv.run();
}

fn show_popup(s: &mut Cursive) {
    s.call_on_name("input", |view: &mut EditView| {
        view.set_content("");
    });
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
