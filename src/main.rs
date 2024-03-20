use lambdaworks_math::{
    elliptic_curve::{
        short_weierstrass::curves::bls12_381::curve::{BLS12381Curve, BLS12381FieldElement},
        traits::IsEllipticCurve,
    },
    traits::ByteConversion,
};

const SECRET_KEY: &str = "0x6C616D6264617370";

fn main() {
    let g = BLS12381Curve::generator();

    let secret = convert_hex_to_scalar(SECRET_KEY);

    //get the public key

}

fn convert_hex_to_scalar(hex: &str) -> BLS12381FieldElement {
    let bytes = hex.trim_start_matches("0x");
    BLS12381FieldElement::from_bytes_be(&bytes).unwrap()
}
