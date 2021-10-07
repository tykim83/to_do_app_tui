use cursive::{With, traits::Boxable, view::Margins, views::{Button, Dialog, DummyView, EditView, LinearLayout, ListView, PaddedView, Checkbox}};

fn main() {

    let done = vec![String::from("Something")];
    let todo = vec![String::from("Else")];

    let mut siv = cursive::default();

    let todo_view = Dialog::around(
        ListView::new()
            .with(
                |c| {
                    for item in done {
                        c.add_child(item.as_str(), Checkbox::new())
                    }
                }
            ))
        .title("ToDo")
        .min_height(10)
        .min_width(25);

    let done_view = Dialog::around(
        ListView::new()
            .with(
                |c| {
                    for item in todo {
                        c.add_child(item.as_str(), Checkbox::new().checked())
                    }
                }
            ))
        .title("Done")
        .min_height(10)
        .min_width(25);

    let add_view = PaddedView::new(
        Margins {
            left: 1,
            right: 1,
            top: 1,
            bottom: 1,
        },
        LinearLayout::horizontal()
            .child(
                EditView::new()
                    .on_submit(|s, text| s.quit())
                    .fixed_width(15),
            )
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
        .button("Quit", |s| s.quit())
        .title("ToDo App"),
    );

    siv.run();
}

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
