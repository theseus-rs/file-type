use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_458: FileFormat = FileFormat {
    id: 458,
    source_type: SourceType::Pronom,
    name: "Aldus Freehand Drawing",
    extensions: &["fh4"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x41, 0x47, 0x44, 0x31]),
                    Token::WildcardCountRange(64, 1_024),
                    Token::Literal(&[
                        0x41, 0x6C, 0x64, 0x75, 0x73, 0x20, 0x46, 0x72, 0x65, 0x65, 0x48, 0x61,
                        0x6E, 0x64, 0x20, 0x34,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 87,
    }],
};
