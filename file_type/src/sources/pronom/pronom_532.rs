use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_532: FileFormat = FileFormat {
    id: 532,
    source_type: SourceType::Pronom,
    name: "TeX/LaTeX Device Independent Document",
    extensions: &["dvi"],
    media_types: &["application/x-dvi"],
    signatures: &[Signature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xF7, 0x02])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xF9]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x02]),
                        Token::WildcardCountRange(0, 3),
                        Token::Literal(&[0xDF, 0xDF, 0xDF, 0xDF]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[],
};
