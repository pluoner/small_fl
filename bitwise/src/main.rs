fn main() {
//implement division without use of neither the multiplication or division operators. Only quota, remainder is discarded. Only possitive numbers. 
    println!("my_div: {}", my_div(1024, 2));
    fn my_div(a: u16, b: u16) -> u16 {
        let mut b_exp = 0;
        while a >= b << b_exp + 1 {
            b_exp += 1;
        }
        let mut cur_a = a;
        let mut quota = 0;
        while b_exp >= 0 {            
            if cur_a >= b << b_exp {
                cur_a -= b << b_exp;
                quota += 1 << b_exp;
                b_exp -= 1;
            } else {
                b_exp -= 1;
            }
        }
        quota
    }

//Given an unsigned 8-bit integer, swap its even and odd bits. The 1st and 2nd bit should be swapped, the 3rd and 4th bit should be swapped, and so on.
//For example, 10101010 should be 01010101. 11100010 should be 11010001.

let my_u8: u8 = 0b11100010;

fn shift_bits(i: u8) -> u8 {
    ((i >> 1) & 85) | ((i << 1) & 170)
}
println!("            my_u8: {:#010b} ({})", my_u8, my_u8);
println!("shift_bits(my_u8): {:#010b} ({})", shift_bits(my_u8), shift_bits(my_u8));

}
