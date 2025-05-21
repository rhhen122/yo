fn main() {
    // Config
    let name = "bro or bruz (:"; // Change what it calls you.
    let credit = false; // Adds Credit from me (:



    let creditmsg = " - Made by Roky Henderson";

    if credit == true {
        yeet(true, name, creditmsg);
    } else {
        yeet(false, name, creditmsg);
    }
}

fn yeet(credit: bool, name: &'static str, creditmsg: &'static str) {
    if credit == true {
        println!("Yo, Hope your doing well {} {}", name, creditmsg);
    }
    if credit == false {
        println!("Yo, Hope your doing well {}", name);
    }
}
