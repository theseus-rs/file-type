use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1594: FileFormat = FileFormat {
    id: 2_421,
    puid: "fmt/1594",
    name: "ESRI ArcInfo DAT File (External)",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2E, 0x2E, 0x2F])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2E, 0x61, 0x64, 0x66])],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_427,
            relationship_type: RelationshipType::EquivalentTo,
        },
        RelatedFormat {
            id: 2_422,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
