use cursive::{
    traits::{Boxable, Nameable},
    view::Margins,
    views::{
        Button, Checkbox, Dialog, DummyView, EditView, LinearLayout, ListView, PaddedView, ViewRef,
    },
    Cursive,
};
mod to_do_list;
use to_do_list::ToDos;

fn main() {
    let mut siv = cursive::default();
    siv.set_user_data(ToDos::new());

    let todo_view = Dialog::around(ListView::new().with_name("todo"))
        .title("ToDo")
        .min_height(10)
        .min_width(25);

    let done_view = Dialog::around(ListView::new().with_name("done"))
        .title("Done")
        .min_height(10)
        .min_width(25);

    let edit_view = EditView::new()
        .on_submit(move |s, text| {
            todo_manager(s, Actions::Add(&text.to_owned()));
            refresh_user_data(s);
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
                todo_manager(s, Actions::Add(&text));
                refresh_user_data(s);
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
            todo_manager(s, Actions::Clear);
            refresh_user_data(s);
        })
        .button("Quit", |s| s.quit())
        .title("ToDo App"),
    );

    refresh_user_data(&mut siv);
    siv.run();
}

fn refresh_user_data(s: &mut Cursive) {
    for (name, status) in [("todo", false), ("done", true)] {
        let mut view: ViewRef<ListView> = s.find_name(name).unwrap();
        view.clear();
        s.with_user_data(|todos: &mut ToDos| {
            todos
                .list
                .iter()
                .filter(|v| *v.1 == status)
                .for_each(|(k, _)| {
                    let mut checkbox = Checkbox::new().on_change(move |s, _| {
                        let text = toogle(s, !status).unwrap();
                        todo_manager(s, Actions::Toggle(&text));
                        refresh_user_data(s);
                    });
                    if status {
                        checkbox.set_checked(status);
                    }
                    view.add_child(k, checkbox);
                });
        });
    }
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

fn todo_manager(s: &mut Cursive, action: Actions) {
    match action {
        Actions::Add(text) => s.user_data().map(|c: &mut ToDos| {
            c.add(text);
        }),
        Actions::Toggle(text) => s.user_data().map(|c: &mut ToDos| {
            c.toogle(text);
        }),
        Actions::Clear => s.user_data().map(ToDos::clear),
    };
}

fn clear_edit_view(s: &mut Cursive) {
    s.call_on_name("input", |c: &mut EditView| {
        c.set_content("");
    });
}
