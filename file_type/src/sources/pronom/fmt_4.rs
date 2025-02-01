use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_4: FileFormat = FileFormat {
    id: 620,
    puid: "fmt/4",
    name: "Graphics Interchange Format",
    extensions: &["gif"],
    media_types: &["image/gif"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x49, 0x46, 0x38, 0x39, 0x61])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3B])],
                },
            },
        ],
    }],
    related_formats: &[RelatedFormat {
        id: 619,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
