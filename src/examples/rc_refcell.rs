/*
Rc and RefCell are two important types in Rust's standard library that help manage ownership 
and borrowing in more complex scenarios. Let me explain each one and then provide a didactic 
example of when and how to use them together.

Rc (Reference Counted):
-----------------------
Rc stands for "Reference Counted." It's a type that allows multiple ownership of the same data. 
When you need to have multiple parts of your code own a piece of data, and you can't determine 
at compile time which part will outlive the others, Rc is useful.

RefCell (Reference Cell):
-------------------------
RefCell provides interior mutability. It allows you to mutate data even when there are immutable 
references to that data, but it moves the borrowing rules from compile-time to runtime.

Now, let's look at an example where we might use both Rc and RefCell together:

Imagine we're building a simple text editor where multiple views can access and modify the same 
document. We'll use Rc to allow multiple ownership of the document, and RefCell to allow mutable 
access to its contents.
*/

use std::rc::Rc;
use std::cell::RefCell;

// Our document structure
struct Document {
    content: String,
}

// A view that can display and edit the document
struct View {
    document: Rc<RefCell<Document>>,
}

impl View {
    fn new(document: Rc<RefCell<Document>>) -> Self {
        View { document }
    }

    fn display(&self) {
        let doc = self.document.borrow();
        println!("Document content: {}", doc.content);
    }

    fn edit(&self, new_content: &str) {
        let mut doc = self.document.borrow_mut();
        doc.content = new_content.to_string();
        println!("Document updated");
    }
}

fn main() {
    // Create a new document
    let document = Rc::new(RefCell::new(Document {
        content: "Hello, world!".to_string(),
    }));

    // Create two views of the same document
    let view1 = View::new(Rc::clone(&document));
    let view2 = View::new(Rc::clone(&document));

    // Display the initial content
    view1.display();

    // Edit the document through view1
    view1.edit("Hello, Rust!");

    // Display the updated content through view2
    view2.display();
}

/*
In this example:

1) We use Rc to allow multiple Views to own the same Document. This is necessary because we 
    don't know which view will be dropped first.

2) We wrap the Document in a RefCell to allow mutable access to its content. This is needed 
    because we want to modify the document through any view, even though we have multiple 
    immutable references to it (via the Rc).

3) The View::display method uses borrow() to get an immutable reference to the document.

4) The View::edit method uses borrow_mut() to get a mutable reference to the document.

5) In main(), we create a single Document and two Views that share ownership of it. We can 
    then edit the document through one view and see the changes reflected in the other.

This pattern is useful when you need shared ownership (hence the Rc) and the ability to 
modify the shared data (hence the RefCell). However, it's important to note that while 
RefCell allows us to bypass Rust's compile-time borrowing rules, it still enforces them 
at runtime. If you try to borrow mutably while an immutable borrow is active, or try to 
have multiple mutable borrows, it will panic at runtime.
*/