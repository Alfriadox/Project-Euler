pub fn main() {
    let total = 1_000_000_000_000f64;

    let blue1 = 3;
    let red1 = 1;
    let mut blue = blue1 as f64;
    let mut red = red1 as f64;

    // consts
    let c1 = blue1 as f64 + red1 as f64*8f64.sqrt();
    let c2 = blue1 as f64 - red1 as f64*8f64.sqrt();

    let mut ind: i32 = 1;

    loop {
        let k = (c1.powi(ind) + c2.powi(ind))/2f64;
        red = ((c1.powi(ind) + c2.powi(ind)) / (2f64*8f64.sqrt()));
        blue = red + ( 1f64 + k ) / 2f64 ;
        if (blue + red) > total {
            println!("{}", blue.ceil() as u64);
            break;
        } else {
            ind += 1;
        }
    }
}
/* math:

(blue/total)*((blue-1)/(total-1)) == 1/2

(blue^2 + blue) / (total^2 + total) == 1/2

2*blue^2 + 2*blue == total^2 +total

2*blue^2 - total^2 +2*blue - total == 0

let total = blue + red

2*blue^2 - (blue^2 + 2*blue*red + red^2) + 2*blue - blue - red == 0

blue^2 - 2*blue*red - red^2 + blue - red == 0

blue^2 - (2*red + 1)*blue - (red^2 -red) == 0  ***

// the determinant (b^2 - 4*a*c) must be a perfect square

(2*red + 1)^2 - 4*( -(red^2 -red)) == k^2

4*red^2 + 4*red + 1 + 4*red^2 - 4*red == k^2

8*red^2 + 1 == k^2

k^2 - 8*red^2 == 1

// this is a Pell equation (http://mathworld.wolfram.com/PellEquation.html) (x^2 - D*y^2 == 1)
    (D == 8)
    therefore: for index n in the convergent solutions

k == ( (3 + 1*sqrt(8))^n + (3 - 1*sqrt(8))^n ) / 2 ; and
red == ( (3 + 1*sqrt(8))^n + (3 - 1*sqrt(8))^n ) / (2*sqrt(8)) ;

// and therefore
blue == red + ( 1 + sqrt( 1 + 8*red^2 ) ) / 2
( via the quadratic equation as k == b^2 - 4*a*c , which is applied to the starred line above )

*/

// 756872327473