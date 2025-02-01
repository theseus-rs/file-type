use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1174: FileFormat = FileFormat {
    id: 1_984,
    puid: "fmt/1174",
    name: "Hewlett Packard Graphics Language",
    extensions: &["000"],
    media_types: &["application/vnd.hp-HPGL"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x1B, 0x25, 0x30, 0x42, 0x42, 0x50, 0x49, 0x4E,
                    ])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x47, 0x3B])],
                },
            },
        ],
    }],
    related_formats: &[RelatedFormat {
        id: 446,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
