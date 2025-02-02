use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1221: FileFormat = FileFormat {
    id: 1_221,
    source_type: SourceType::Pronom,
    name: "AutoCAD Drawing",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x43, 0x31, 0x30, 0x32, 0x34, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 766,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_318,
        },
    ],
};
