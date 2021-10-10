use std::cell::RefCell;
use std::rc::Rc;

use cursive::{
    traits::{Boxable, Nameable},
    view::Margins,
    views::{Button, Checkbox, Dialog, DummyView, EditView, LinearLayout, ListView, PaddedView},
    Cursive,
};
mod to_do_list;
use to_do_list::ToDos;

fn main() {
    let todos = Rc::new(RefCell::new(ToDos::new()));

    let mut siv = cursive::default();

    let todo_view = Dialog::around(ListView::new().with_name("todo"))
        .title("ToDo")
        .min_height(10)
        .min_width(25);

    let done_view = Dialog::around(ListView::new().with_name("done"))
        .title("Done")
        .min_height(10)
        .min_width(25);

    let todo_add_button = todos.clone();
    let todo_clear_button = todos.clone();
    let todo_edit_view = todos.clone();
    let todo_first_refresh = todos;

    let edit_view = EditView::new()
        .on_submit(move |s, text| {
            todo_manager(Actions::Add(&text.to_owned()), todo_edit_view.clone());
            refresh(s, todo_edit_view.clone());
            clear_edit_view(s);
        })
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
            .child(Button::new("Add", move |s| {
                let text = s
                    .call_on_name("input", |c: &mut EditView| c.get_content())
                    .unwrap();
                todo_manager(Actions::Add(&text), todo_add_button.clone());
                refresh(s, todo_add_button.clone());
                clear_edit_view(s);
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
        .button("Clear", move |s| {
            todo_manager(Actions::Clear, todo_clear_button.clone());
            refresh(s, todo_clear_button.clone());
        })
        .button("Quit", |s| s.quit())
        .title("ToDo App"),
    );

    refresh(&mut siv, todo_first_refresh);
    siv.run();
}

enum Actions<'a> {
    Add(&'a str),
    Toggle(&'a str),
    Clear,
}

fn toogle(s: &mut Cursive, checked: bool) -> Option<String> {
    if checked {
        s.call_on_name("todo", |list: &mut ListView| {
            let index = list.focus();
            let child = list.get_row(index);
            match child {
                cursive::views::ListChild::Row(label, _) => Some(label.clone()),
                cursive::views::ListChild::Delimiter => None,
            }
        })
        .unwrap()
    } else {
        s.call_on_name("done", |list: &mut ListView| {
            let index = list.focus();
            let child = list.get_row(index);
            match child {
                cursive::views::ListChild::Row(label, _) => Some(label.clone()),
                cursive::views::ListChild::Delimiter => None,
            }
        })
        .unwrap()
    }
}

fn todo_manager(action: Actions, td: Rc<RefCell<ToDos>>) {
    let td = td.as_ref();
    match action {
        Actions::Add(text) => td.borrow_mut().add(text),
        Actions::Toggle(text) => td.borrow_mut().toogle(text),
        Actions::Clear => td.borrow_mut().clear(),
    }
}

fn refresh(s: &mut Cursive, td: Rc<RefCell<ToDos>>) {
    for (name, status) in [("todo", false), ("done", true)] {
        s.call_on_name(name, |list: &mut ListView| {
            list.clear();
            for (k, _) in td.as_ref().borrow().list.iter().filter(|v| *v.1 == status) {
                let td = td.clone();
                let mut checkbox = Checkbox::new().on_change(move |s, checked| {
                    let text = toogle(s, checked).unwrap();
                    todo_manager(Actions::Toggle(&text), td.clone());
                    refresh(s, td.clone());
                });
                if status {
                    checkbox.set_checked(status);
                }
                list.add_child(
                    k,
                    checkbox
                );
            }
        });
    }
}

fn clear_edit_view(s: &mut Cursive) {
    s.call_on_name("input", |c: &mut EditView| {
        c.set_content("");
    });
}
