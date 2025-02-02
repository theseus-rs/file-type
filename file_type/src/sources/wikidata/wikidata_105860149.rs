use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860149: FileFormat = FileFormat {
    id: 105_860_149,
    source_type: SourceType::Wikidata,
    name: "RoboForm saved data",
    extensions: &["rfn", "rfp", "rfx"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x55, 0x52, 0x4C, 0x33, 0x3A, 0x76, 0x65, 0x72, 0x33, 0x3A,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x55, 0x52, 0x4C, 0x33, 0x3A, 0x76, 0x65, 0x72, 0x33, 0x3A,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x55, 0x52, 0x4C, 0x33, 0x3A, 0x76, 0x65, 0x72, 0x33, 0x3A,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
