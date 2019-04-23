// test1.rs
// This is a test for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 dollars, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount. No hints this time!

// Put your function here!
fn calculateprice(num_apples: u32) -> u32 {
    if num_apples <= 40 {
        num_apples * 2
    } else {
        num_apples
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculateprice(55);
    let price2 = calculateprice(40);

    assert_eq!(price1, 55);
    assert_eq!(price2, 80);

    // Ha ha! Modifying it anyway!
    let price3 = calculateprice(39);
    let price4 = calculateprice(41);
    assert_eq!(price3, 78);
    assert_eq!(price4, 41);
}
