use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_661: FileFormat = FileFormat {
    id: 661,
    source_type: SourceType::Pronom,
    name: "Virtual Reality Modeling Language",
    extensions: &["wrl"],
    media_types: &["model/vrml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x56, 0x52, 0x4D, 0x4C, 0x20, 0x56, 0x31, 0x2E, 0x30, 0x20, 0x61, 0x73,
                    0x63, 0x69, 0x69,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 662,
    }],
};
