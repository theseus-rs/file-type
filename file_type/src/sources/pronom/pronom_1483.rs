use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1483: FileFormat = FileFormat {
    id: 1_483,
    source_type: SourceType::Pronom,
    name: "Vectorworks",
    extensions: &["vwx"],
    media_types: &["application/vnd.vectorworks"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x00, 0x00, 0x08, 0xA8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94,
                        0x00, 0x20, 0x56, 0x57, 0x32, 0x30, 0x2E, 0x30,
                    ]),
                    Token::WildcardCountRange(0, 3),
                    Token::Literal(&[0x56, 0x57, 0x32, 0x30, 0x2E, 0x30]),
                    Token::WildcardCountRange(0, 29),
                    Token::Literal(&[
                        0x56, 0x65, 0x63, 0x74, 0x6F, 0x72, 0x57, 0x6F, 0x72, 0x6B, 0x73,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
