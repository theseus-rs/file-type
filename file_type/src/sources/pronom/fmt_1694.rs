use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1694: FileFormat = FileFormat {
    id: 2_530,
    puid: "fmt/1694",
    name: "Asymetrix Compel Presentation",
    extensions: &["cpl", "art"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x03, 0x4A, 0x42, 0x4F, 0x4F, 0xF5, 0x3C, 0x55,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_529,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
