use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_319: FileFormat = FileFormat {
    id: 478,
    puid: "x-fmt/319",
    name: "FileMaker Pro Database",
    extensions: &["fp5", "fmp", "fp", "fm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x05, 0x00, 0x02,
                        0x00, 0x02, 0xC0,
                    ]),
                    Token::WildcardCount(527),
                    Token::Literal(&[0x50, 0x72, 0x6F, 0x20, 0x35]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
