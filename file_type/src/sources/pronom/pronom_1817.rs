use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1817: FileFormat = FileFormat {
    id: 1_817,
    source_type: SourceType::Pronom,
    name: "INTERLIS Model File",
    extensions: &["ili"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x49, 0x4E, 0x54, 0x45, 0x52, 0x4C, 0x49, 0x53, 0x20, 0x32, 0x2E, 0x32,
                        0x3B,
                    ]),
                    Token::WildcardCountRange(1, 1_024),
                    Token::Literal(&[0x4D, 0x4F, 0x44, 0x45, 0x4C]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
