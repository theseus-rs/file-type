use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1375: FileFormat = FileFormat {
    id: 1_375,
    source_type: SourceType::Pronom,
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
