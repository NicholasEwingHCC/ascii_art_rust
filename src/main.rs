use image::*;

fn main() {
    let char_vec = vec!['`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '~', '+', '_', '-', '?', ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v', 'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k', 'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$'];

    let img = image::io::Reader::open(&std::path::Path::new("images/download.jpeg")).unwrap().decode().unwrap().to_luma8();
    let (width, height) = img.dimensions();

    let mut h_counter = 0;
    let mut w_counter = 0;
    while h_counter < height {
        let mut answer: Vec<char> = Vec::new();
        while w_counter < width {
            let val = img.get_pixel(w_counter, h_counter);
            let c_index_adjust: u8 = 4;
            let mut c_index = (val[0] / c_index_adjust) as usize;

            if c_index > 64 {
                c_index = 64;
            }

            answer.push(char_vec[c_index]);
            answer.push(char_vec[c_index]);

            w_counter += 1;
        }

        h_counter += 1;
        w_counter = 0;
        let mut char_counter = 0;
        while char_counter < answer.len() {
            print!("{}", answer[char_counter]);
            char_counter += 1;
        }
        print!("\n");
    }
}