use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855813: FileFormat = FileFormat {
    id: 105_855_813,
    source_type: SourceType::Wikidata,
    name: "Zinc Data",
    extensions: &["dat", "z_t", "znc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5A, 0x69, 0x6E, 0x63, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20, 0x46, 0x69,
                        0x6C, 0x65,
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
                        0x5A, 0x69, 0x6E, 0x63, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20, 0x46, 0x69,
                        0x6C, 0x65,
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
                        0x5A, 0x69, 0x6E, 0x63, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20, 0x46, 0x69,
                        0x6C, 0x65,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
