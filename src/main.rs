use colored::Colorize;
fn main() {
    // Config
    let name = "bro or bruz (:"; // Change what it calls you.
    let credit = true; // Adds Credit from me (:
    let color = true; // Makes the 'Yo, ' Yellow.



    let creditmsg = "☁️"; // Change the credit, the thing at the end.

    if credit == true {
        if color == true {
            yeet(true, true, name, creditmsg);
        } else {
            yeet(false, false, name, creditmsg);
        }
    } else {
        if color == true {
            yeet(true, false, name, creditmsg);
        } else {
            yeet(false, false, name, creditmsg);
        }
    }
}

fn yeet(color: bool, credit: bool, name: &'static str, creditmsg: &'static str) {
    if credit == true {
        if color == true {
            println!("{}Hope your doing well {} {}", "Yo, ".yellow(), name, creditmsg);
        } else {
            println!("Yo, Hope your doing well {}", name);
        }
    }
    if credit == false {
        if color == true {
            println!("{}Hope your doing well {}", "Yo, ".yellow(), name);
        } else {
            println!("Yo, Hope your doing well {}", name);
        }
        
    }
}


/* Thanks for looking through my terrible code
I know its basically just a bunch of nested if statements

*/