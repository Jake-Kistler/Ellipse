use std::io;

// let is a keyword for declaration of a variable, by default in rust they are non mutable meaning they can't change once set
// mut is a keyword for mutable variables, becurse of this input can be changed during the run time

fn calculate_distance_between_foci_points(pair1: (f32, f32), pair2: (f32, f32)) -> f32 {
    let (x1, y1) = pair1; // doing this we can access the x and y component separately
    let (x2, y2) = pair2;

    // calculate the distance between the two points distance = sqrt((x2-x1)^2 + (y2-y1)^2) / 2
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt() // this is the return statement as it DOES NOT have a ;, you can explicitly place return if I wanted

}

// calculate the semi minor axis 
fn calculate_semi_minor_axis(semi_major_axis: f32, distance: f32) -> f32 { 
     (semi_major_axis.powi(2) - (distance / 2.0).powi(2)).sqrt() // semi_minor = sqrt(Semi_major^2 - distance^2)
}


fn main() {
    // Create a string that will hold the input
    let mut input = String::new();

    // Prompt the user
    println!("Enter data: (x1 y1 x2 y2 a) where a is the length of the major axis");

    // read the data from the user
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Trim any leading or exiting whitespace from the input string
    let input = input.trim();

    // split the input string based on the enteral whitespace characters
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() == 5 {
        // parse and group data 
        let x1: f32 = parts[0].parse().expect("Please enter a valid number for x1");
        let y1: f32 = parts[1].parse().expect("Please enter a valid number for y1");
        let x2: f32 = parts[2].parse().expect("Please enter a valid number for x2");
        let y2: f32 = parts[3].parse().expect("Please enter a valid number for y2");
        let major_axis: f32 = parts[4].parse().expect("Please enter a valid number for the major axis");

        // Group the x and y parts into tuples
        let pair1 = (x1, y1);
        let pair2 = (x2, y2);

        let semi_major_axis = major_axis / 2.0;

        let distance = calculate_distance_between_foci_points(pair1, pair2);

        let semi_minor_axis = calculate_semi_minor_axis(semi_major_axis, distance);

        // Calculate the center of the ellipse
        let x_center = (x1 + x2) / 2.0;
        let y_center = (y1 + y2) / 2.0;

        // Calculate the bounding box
        let x_low = x_center - semi_major_axis;
        let y_low = y_center - semi_minor_axis;
        let x_high = x_center + semi_major_axis;
        let y_high = y_center + semi_minor_axis;

        // print the bounding box
        println!("{:.6} {:.6} {:.6} {:.6}", x_low, y_low, x_high, y_high);
    }
    else{
        println!("Invalid input, lacking 5 values...");
    }

   
}
