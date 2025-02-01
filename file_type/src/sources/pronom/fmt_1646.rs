use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1646: FileFormat = FileFormat {
    id: 2_473,
    puid: "fmt/1646",
    name: "Roxio Label Creator Project File",
    extensions: &["jwl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x00, 0x4F, 0x00, 0x58, 0x00, 0x49, 0x00, 0x20,
                    ])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(109),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x02, 0xFF, 0xFE, 0xFF, 0x05, 0x41, 0x00, 0x72, 0x00, 0x69, 0x00, 0x61,
                        0x00, 0x6C,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[RelatedFormat {
        id: 2_472,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
