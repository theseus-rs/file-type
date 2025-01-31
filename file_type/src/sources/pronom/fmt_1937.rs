use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1937: FileFormat = FileFormat {
    id: 2_798,
    puid: "fmt/1937",
    name: "Amazon Kindle eBook File",
    extensions: &["azw", "azw3", "mobi", "amr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(60),
            regex: Regex {
                tokens: &[
                    Token::Any(&[
                        &[Token::Literal(&[
                            0x42, 0x4F, 0x4F, 0x4B, 0x4D, 0x4F, 0x42, 0x49,
                        ])],
                        &[Token::Literal(&[
                            0x54, 0x45, 0x58, 0x74, 0x52, 0x45, 0x41, 0x64,
                        ])],
                    ]),
                    Token::WildcardCountRange(450, 15_000),
                    Token::Literal(&[0x6B, 0x69, 0x6E, 0x64, 0x6C, 0x65, 0x3A]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_144,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
