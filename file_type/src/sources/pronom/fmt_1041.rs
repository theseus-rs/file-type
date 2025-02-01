use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1041: FileFormat = FileFormat {
    id: 1_846,
    puid: "fmt/1041",
    name: "HDF",
    extensions: &["hdf", "h4"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0E, 0x03, 0x13, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
