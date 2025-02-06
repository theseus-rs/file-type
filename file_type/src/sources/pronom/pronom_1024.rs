use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1024: FileFormat = FileFormat {
    id: 1_024,
    source_type: SourceType::Pronom,
    name: "Gridded Binary",
    extensions: &["grb", "wmo"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x47, 0x52, 0x49, 0x42]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x01]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x37, 0x37, 0x37, 0x37])],
                },
            },
        ],
    }],
    related_formats: &[],
};
