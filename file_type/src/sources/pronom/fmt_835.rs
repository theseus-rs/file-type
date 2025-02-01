use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_835: FileFormat = FileFormat {
    id: 1_636,
    puid: "fmt/835",
    name: "Quattro Pro Spreadsheet for Windows",
    extensions: &["wb2"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x02, 0x00, 0x02, 0x10])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_637,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_635,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
