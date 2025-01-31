use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_282: FileFormat = FileFormat {
    id: 1_022,
    puid: "fmt/282",
    name: "netCDF-3 Classic",
    extensions: &["nc", "cdf"],
    media_types: &["application/netcdf", "application/x-netcdf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x44, 0x46, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
