use crate::twenty_fifteen::file_string::file_string;

// You need to determine the total square feet of wrapping paper to order given an input of a file
// containing the dimensions ofthe gifts
// 2*l*w + 2*w*h + 2*h*l given the surface area of a recetangular prisim
// You want to add extra wrapping paper for each box just in case add area of smallest size
//
// Read in file and convert to string 
// convert string to array of strings, each index representing a dimension of a present
// For each index determine the area of the 3 sides, calculate surface area + smallest side area
// Add amount of paper needed for that one present to total, continue doing for whole list
// print total paper order needed
pub fn two() {
    let path = String::from("./twenty_fifteen/two.txt");
    let s = file_string(path);

    let presents: Vec<&str> = s.lines().collect();
    let mut total_paper = 0;
    let mut total_ribbon = 0;
    for present in presents {
        let dimensions: Vec<&str> = present.split(char::is_alphabetic).collect();
        let (l, w, h) = (dimensions[0], dimensions[1], dimensions[2]);
        let l: i32 = l.parse().unwrap();
        let w: i32 = w.parse().unwrap();
        let h: i32 = h.parse().unwrap();
        let (area1, area2, area3) = (l*w, w*h, h*l);
        let areas: [i32; 3] = [area1, area2, area3];
        let min_area = areas.iter().min().unwrap();
        let paper = (2*(area1 + area2 + area3)) + min_area;
        let perimeters:[i32; 3] = [2*(l+w), 2*(w+h), 2*(h+l)];
        let min_perimeter = perimeters.iter().min().unwrap();
        let ribbon = (min_perimeter) + (l*w*h);
        total_paper += paper;
        total_ribbon += ribbon;
        println!("{}", total_paper);
    }
    println!("{}, {}", total_paper, total_ribbon);
}

