use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1023: FileFormat = FileFormat {
    id: 1_023,
    source_type: SourceType::Pronom,
    name: "netCDF-3 64-bit",
    extensions: &["nc", "cdf"],
    media_types: &["application/netcdf", "application/x-netcdf"],
    signatures: &[Signature {
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
