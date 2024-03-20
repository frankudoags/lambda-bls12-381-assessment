use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::{
        short_weierstrass::curves::bls12_381::curve::BLS12381Curve, traits::IsEllipticCurve,
    },
};

const SECRET_KEY: &str = "0x6C616D6264617370";

fn main() {
    let g = BLS12381Curve::generator();

    let secret = convert_hex_to_unsigned_int(SECRET_KEY);

    //get the public key
    let public = g.operate_with_self(secret);
    println!(
        "Public key: X: {:#?},\n Y: {:#?}",
        limbs_to_hex(&public.coordinates()[0].value().limbs),
        limbs_to_hex(&public.coordinates()[1].value().limbs),
    );

    //Public key: X: "13f9a0ebc51ea42000505bc4fc614b0439a11c572fc9deff89da9a3d2b8cfa5f0103be2054cb8bcd4147382c0c9f36bc",
    // Y: "0ef70d419e63b8691779b871967705a573269bb3f14a0b9973da407c1a8acce5507ef262e44e9d6c56d3508f80dba43f"
}

/// This converts a hex string to an unsigned integer, we use this to convert the secret key to an integer
/// so that we can use it to generate the public key using the elliptic curve
/// multiplication i.e we multiply the generator point by the secret key to get the public key
fn convert_hex_to_unsigned_int(hex: &str) -> u128 {
    u128::from_str_radix(&hex[2..], 16).unwrap()
}

/// This converts the limbs of the X and Y Field elements into a hex string
fn limbs_to_hex(limbs: &[u64]) -> String {
    limbs
        .iter()
        .map(|&limb| format!("{:016x}", limb))
        .collect::<Vec<String>>()
        .join("")
}
