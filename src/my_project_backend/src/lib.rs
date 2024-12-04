use std::cell::RefCell;

thread_local! {
    static CHAT: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
fn add_msg(new_msg: String) {
    
    CHAT.with(|chat| {
        chat.borrow_mut().push(new_msg)
    })
}

#[ic_cdk::query]
fn get_chat() -> Vec<String> {
    CHAT.with(|chat| chat.borrow().clone())
}

#[ic_cdk::query]
fn get_msg(name: String) -> String {
    let a = String::from("Siema");
    let b = &a;
    let c = a;

    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    let a = String::from("Siema");
    let b = &a;
    let c = a;

    format!("Hello, {}!", name)
}
