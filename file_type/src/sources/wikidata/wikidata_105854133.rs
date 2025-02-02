use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854133: FileFormat = FileFormat {
    id: 105_854_133,
    source_type: SourceType::Wikidata,
    name: "ZyXEL Voice Format audio",
    extensions: &["ad2", "zvd", "zyx"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x79, 0x58, 0x45, 0x4C, 0x02])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x79, 0x58, 0x45, 0x4C, 0x02])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x79, 0x58, 0x45, 0x4C, 0x02])],
                },
            }],
        },
    ],
    related_formats: &[],
};
