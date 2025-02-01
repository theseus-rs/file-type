use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1793: FileFormat = FileFormat {
    id: 2_643,
    puid: "fmt/1793",
    name: "ICDRAW Group Icon File",
    extensions: &["ib3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x43, 0x42, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
