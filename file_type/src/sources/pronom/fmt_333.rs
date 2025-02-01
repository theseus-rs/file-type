use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_333: FileFormat = FileFormat {
    id: 1_078,
    puid: "fmt/333",
    name: "Chemical Markup Language",
    extensions: &["cml"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3D, 0x22, 0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F,
                    0x2F, 0x77, 0x77, 0x77, 0x2E, 0x78, 0x6D, 0x6C, 0x2D, 0x63, 0x6D, 0x6C, 0x2E,
                    0x6F, 0x72, 0x67, 0x2F, 0x73, 0x63, 0x68, 0x65, 0x6D, 0x61, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 638,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
