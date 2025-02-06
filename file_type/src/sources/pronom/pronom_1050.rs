use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1050: FileFormat = FileFormat {
    id: 1_050,
    source_type: SourceType::Pronom,
    name: "Computer Graphics Metafile (Binary)",
    extensions: &["cgm"],
    media_types: &["image/cgm; version=3"],
    signatures: &[Signature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00]),
                        Token::Range(&[0x20], &[0x3F]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x10, 0x22, 0x00, 0x03]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x40])],
                },
            },
        ],
    }],
    related_formats: &[],
};
