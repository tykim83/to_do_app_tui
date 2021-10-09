use std::borrow::BorrowMut;
use std::rc::Rc;
use std::thread::AccessError;
use std::{borrow::Borrow, cell::RefCell};

use cursive::backend::Backend;
use cursive::{
    traits::{Boxable, Nameable},
    view::Margins,
    views::{
        Button, Checkbox, Dialog, DummyView, EditView, LinearLayout, ListChild, ListView,
        PaddedView,
    },
    Cursive, With,
};
mod to_do_list;
use to_do_list::ToDos;

fn main() {
    let todos = RefCell::new(ToDos::new());

    let mut siv = cursive::default();

    let todo_view = Dialog::around(ListView::new().with(|list| {
        for (k, _) in todos.borrow().list.iter().filter(|v| !*v.1) {
            list.add_child(k, Checkbox::new());
        }
    }).with_name("todo"))
        .title("ToDo")
        .min_height(10)
        .min_width(25);

    let done_view = Dialog::around(ListView::new().with(|list| {
        for (k, _) in todos.borrow().list.iter().filter(|v| *v.1) {
            list.add_child(k, Checkbox::new().checked());
        }
    }).with_name("done"))
        .title("Done")
        .min_height(10)
        .min_width(25);

    let todo_manager = move |text: &str, action: Actions| {
        match action {
            Actions::Add => todos.borrow_mut().add(text),
            Actions::Toggle => todo!(),
            Actions::Clear => todo!(),
        }
    };

    let refresh = move |s: &mut Cursive| {
        s.call_on_name("todo", |list: &mut ListView| {
            for (k, _) in todos.borrow().list.iter().filter(|v| !*v.1) {
                list.add_child(k, Checkbox::new());
            }
        });
        s.call_on_name("done", |list: &mut ListView| {
            for (k, _) in todos.borrow().list.iter().filter(|v| *v.1) {
                list.add_child(k, Checkbox::new().checked());
            }
        });
    };

    let edit_view = EditView::new().on_submit( move |s, text| {
        todo_manager(text.clone(), Actions::Add);
        refresh(s);
    }).with_name("input").fixed_width(15);

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
            // .child(Button::new("Add", |s| {
            //     let text = s
            //         .call_on_name("input", |c: &mut EditView| c.get_content())
            //         .unwrap();
            //     add_todo(s, text.as_str());
            // }))
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
            s.call_on_name("done", |c: &mut ListView| c.clear());
        })
        .button("Quit", |s| s.quit())
        .title("ToDo App"),
    );

    siv.run();
}

enum Actions {
    Add,
    Toggle,
    Clear,
}

// fn add_todo(text: &str, todos: & mut ToDos) {
//     todos.list.push(ToDo { name: text.to_string(), is_done: false});
// }

// fn refresh<'a>(s: &mut Cursive, todos: &'a Rc<RefCell<ToDos>>) {
//     s.call_on_name("done", |list: &mut ListView| {
//         list.clear();

//         let here: &RefCell<ToDos> = Rc::clone(&todos).borrow();
//         for item in here.borrow().list.iter().filter(|v| v.is_done) {
//             list.add_child(item.name.as_str(), Checkbox::new().on_change(|siv: &mut Cursive, checked:bool| {
//                 toogle_todo(siv, checked);
//             }));
//         }
//     });

//     s.call_on_name("todo", |list: &mut ListView| {
//         list.clear();
//         let here: &RefCell<ToDos> = Rc::clone(&todos).borrow();
//         for item in here.borrow().list.iter().filter(|v| !v.is_done) {
//             list.add_child(item.name.as_str(), Checkbox::new().on_change(|siv: &mut Cursive, checked:bool| {
//                 toogle_todo(siv, checked);
//             }));
//         }
//     });
// }

// fn toogle_todo<'a>(s: &mut Cursive, checked: bool) {
//     let label = match checked {
//         true => get_label(s, "todo").unwrap(),
//         false => get_label(s, "done").unwrap(),
//     };

//     //todos.toggle(&label);
// }

// fn get_label(s: &mut Cursive, name: &str) -> Option<String> {
//     s.call_on_name(name, |c: &mut ListView| {
//         let index = c.focus();
//         let child = c.get_row(index);
//         match child {
//             ListChild::Row(label, _) => label.clone(),
//             ListChild::Delimiter => todo!(),
//         }
//     })
// }

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
