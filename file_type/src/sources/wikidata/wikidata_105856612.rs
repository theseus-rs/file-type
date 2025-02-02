use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856612: FileFormat = FileFormat {
    id: 105_856_612,
    source_type: SourceType::Wikidata,
    name: "Settlers II map",
    extensions: &["swd", "wld"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x4F, 0x52, 0x4C, 0x44, 0x5F, 0x56, 0x31, 0x2E, 0x30,
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
                        0x57, 0x4F, 0x52, 0x4C, 0x44, 0x5F, 0x56, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
