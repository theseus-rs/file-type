use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855729: FileFormat = FileFormat {
    id: 105_855_729,
    source_type: SourceType::Wikidata,
    name: "CompuServe Information Manager DB key/index",
    extensions: &["dat", "db"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x49, 0x4D, 0x20, 0x4B, 0x45, 0x59, 0x46, 0x49, 0x4C, 0x45, 0x1A,
                    ])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x49, 0x4D, 0x20, 0x4B, 0x45, 0x59, 0x46, 0x49, 0x4C, 0x45, 0x1A,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
