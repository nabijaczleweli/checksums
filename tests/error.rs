

use checksums::Error;


#[test]
fn exit_value() {
    assert_eq!(Error::NoError.exit_value(), 0);
    assert_eq!(Error::OptionParsingError.exit_value(), 1);
    assert_eq!(Error::HashLengthDiffers.exit_value(), 2);
    assert_eq!(Error::HashesFileParsingFailure.exit_value(), 3);
    assert_eq!(Error::NFilesDiffer(1).exit_value(), 4);
    assert_eq!(Error::NFilesDiffer(10).exit_value(), 13);
}

#[test]
fn from_i32() {
    assert_eq!(Error::from(0), Error::NoError);
    assert_eq!(Error::from(1), Error::OptionParsingError);
    assert_eq!(Error::from(2), Error::HashLengthDiffers);
    assert_eq!(Error::from(3), Error::HashesFileParsingFailure);
    assert_eq!(Error::from(4), Error::NFilesDiffer(1));
    assert_eq!(Error::from(13), Error::NFilesDiffer(10));
}
