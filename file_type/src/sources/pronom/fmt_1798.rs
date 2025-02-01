use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1798: FileFormat = FileFormat {
    id: 2_649,
    puid: "fmt/1798",
    name: "CHAT Transcription Format",
    extensions: &["cha"],
    media_types: &["text/x-chat"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x40, 0x55, 0x54, 0x46, 0x38]),
                    Token::WildcardCountRange(0, 256),
                    Token::Literal(&[0x40, 0x42, 0x65, 0x67, 0x69, 0x6E]),
                    Token::WildcardCountRange(0, 64),
                    Token::Literal(&[
                        0x40, 0x4C, 0x61, 0x6E, 0x67, 0x75, 0x61, 0x67, 0x65, 0x73, 0x3A,
                    ]),
                    Token::WildcardCountRange(0, 256),
                    Token::Literal(&[
                        0x40, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x69, 0x70, 0x61, 0x6E, 0x74,
                        0x73, 0x3A,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
