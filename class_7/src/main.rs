fn main() {
    let cond = (2 as f32) <= 2.2;
    println!("{}", cond);

    let cond2 = true && cond;
    println!("{}", cond2);
    

    {
        let food = "fruit";

        if food == "cookie" {
            println!("I like cookies too!");
        } else if food == "fruit" {
            println!("That sounds healthy!");
        } else {
            println!("I dont't like cookies and fruits!");
        }
    }
}
