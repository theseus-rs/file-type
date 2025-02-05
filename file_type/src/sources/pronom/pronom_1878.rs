use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1878: FileFormat = FileFormat {
    id: 1_878,
    source_type: SourceType::Pronom,
    name: "Apple Disk Image",
    extensions: &["dmg"],
    media_types: &["application/x-apple-diskimage"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::EOF,
            offset: Some(500),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6B, 0x6F, 0x6C, 0x79, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x02, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 388,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_586,
        },
    ],
};
