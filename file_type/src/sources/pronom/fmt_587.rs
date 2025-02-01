use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_587: FileFormat = FileFormat {
    id: 1_375,
    puid: "fmt/587",
    name: "LifeTechnologies ABIF",
    extensions: &["abif"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x42, 0x49, 0x46, 0x00, 0x65])],
            },
        }],
    }],
    related_formats: &[],
};
