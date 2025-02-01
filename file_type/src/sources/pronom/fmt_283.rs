use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_283: FileFormat = FileFormat {
    id: 1_023,
    puid: "fmt/283",
    name: "netCDF-3 64-bit",
    extensions: &["nc", "cdf"],
    media_types: &["application/netcdf", "application/x-netcdf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x44, 0x46, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
