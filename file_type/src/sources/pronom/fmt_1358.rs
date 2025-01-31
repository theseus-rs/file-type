use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1358: FileFormat = FileFormat {
    id: 2_176,
    puid: "fmt/1358",
    name: "MicroStation Base File",
    extensions: &["bse"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
        id: 2_374,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
