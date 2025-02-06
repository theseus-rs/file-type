use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207261: FileFormat = FileFormat {
    id: 28_207_261,
    source_type: SourceType::Wikidata,
    name: "Seattle FilmWorks",
    extensions: &["alb", "pwm", "pwp", "sfw"],
    media_types: &[],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x46, 0x57, 0x5F, 0x41, 0x4C, 0x42])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x46, 0x57, 0x5F, 0x41, 0x4C, 0x42])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x46, 0x57, 0x5F, 0x41, 0x4C, 0x42])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x46, 0x57, 0x5F, 0x41, 0x4C, 0x42])],
                },
            }],
        },
    ],
    related_formats: &[],
};
