use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_340: FileFormat = FileFormat {
    id: 504,
    puid: "x-fmt/340",
    name: "Lotus WordPro Document",
    extensions: &["lwp"],
    media_types: &["application/lwp", "application/vnd.lotus-wordpro"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x57, 0x6F, 0x72, 0x64, 0x50, 0x72, 0x6F, 0x0D, 0xFB, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x98, 0x5C, 0x81, 0x72, 0x03, 0x00,
                        0x40, 0xCC, 0xC1, 0xBF, 0xFF, 0xBD, 0xF9,
                    ]),
                    Token::Any(&[&[Token::Literal(&[0x50])], &[Token::Literal(&[0x70])]]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_085,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
