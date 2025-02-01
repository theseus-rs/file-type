use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_305: FileFormat = FileFormat {
    id: 1_050,
    puid: "fmt/305",
    name: "Computer Graphics Metafile (Binary)",
    extensions: &["cgm"],
    media_types: &["image/cgm; version=3"],
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
