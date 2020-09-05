#[macro_use]
extern crate yew;

use yew::html::*;

struct Model {
    input: String,
    todos: Vec<String>,
}

enum Msg {
    Add,
    Update(String),
    Remove(usize),
    RemoveAll,
    Nothing,
}

fn update(_: &mut Context<Msg>, model: &mut Model, msg: Msg){
    match msg {
        Msg::Add => {}
        Msg::Update(s) => {}
        Msg::Remove(i) => {}
        Msg::RemoveAll => {}
        Msg::Nothing => {}
    }
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        
    }
}

fn main(){
    println!("Hello, World");
}