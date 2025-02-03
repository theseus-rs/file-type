use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1048: FileFormat = FileFormat {
    id: 1_048,
    source_type: SourceType::Pronom,
    name: "Computer Graphics Metafile (Binary)",
    extensions: &["cgm"],
    media_types: &["image/cgm; version=1"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00]),
                        Token::Range(&[0x20], &[0x3F]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x10, 0x22, 0x00, 0x01]),
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
