use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2176: FileFormat = FileFormat {
    id: 2_176,
    source_type: SourceType::Pronom,
    name: "MicroStation Base File",
    extensions: &["bse"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(824),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x48, 0x50, 0x54, 0x5F, 0x44, 0x44, 0x5F, 0x49, 0x47, 0x44, 0x53, 0x3A,
                        0x46, 0x4F, 0x4E, 0x54, 0x4C, 0x49, 0x42,
                    ]),
                    Token::WildcardCountRange(32_600, 33_200),
                    Token::Literal(&[0x44, 0x47, 0x4E, 0x2D, 0x54, 0x69, 0x74, 0x6C, 0x65, 0x73]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 2_374,
    }],
};
