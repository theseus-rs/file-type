use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1065: FileFormat = FileFormat {
    id: 1_065,
    source_type: SourceType::Pronom,
    name: "ESRI Shapefile Projection (Well-Known Text) Format",
    extensions: &["prj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x47, 0x45, 0x4F, 0x47, 0x43, 0x53]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x44, 0x41, 0x54, 0x55, 0x4D]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x53, 0x50, 0x48, 0x45, 0x52, 0x4F, 0x49, 0x44]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x50, 0x52, 0x49, 0x4D, 0x45, 0x4D]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x55, 0x4E, 0x49, 0x54]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5D, 0x5D])],
                },
            },
        ],
    }],
    related_formats: &[],
};
