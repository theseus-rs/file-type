use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1277: FileFormat = FileFormat {
    id: 2_095,
    puid: "fmt/1277",
    name: "Cindex Document",
    extensions: &["cdx", "tpl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x18, 0x24, 0x00, 0x00, 0xCB, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_096,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
