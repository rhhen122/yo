use colored::Colorize;
fn main() {
    // Config
    let name = "bro or bruz (:"; // Change what it calls you.
    let credit = false; // Adds Credit from me (:
    let color = true; // Makes the 'Yo, ' Yellow.



    let creditmsg = " - Made by Roky Henderson";

    if credit == true {
        if color == true {
            yeet(true, true, name, creditmsg);
        } else {
            println!("Yo, Hope your doing well {}", name);
        }
    } else {
        if color == true {
            yeet(true, false, name, creditmsg);
        } else {
            println!("Yo, Hope your doing well {}", name);
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
