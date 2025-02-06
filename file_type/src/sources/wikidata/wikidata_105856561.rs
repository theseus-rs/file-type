use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856561: FileFormat = FileFormat {
    id: 105_856_561,
    source_type: SourceType::Wikidata,
    name: "eBeam Whiteboard",
    extensions: &["esb", "wbd"],
    media_types: &[],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x65, 0x42, 0x65, 0x61, 0x6D, 0x4D, 0x65, 0x65, 0x74, 0x69, 0x6E, 0x67,
                        0x46, 0x69, 0x6C, 0x65, 0x0D, 0x0A, 0x4F, 0x72, 0x69, 0x67, 0x69, 0x6E,
                        0x61, 0x6C, 0x20, 0x43, 0x72, 0x65, 0x61, 0x74, 0x6F, 0x72, 0x3A,
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
                        0x65, 0x42, 0x65, 0x61, 0x6D, 0x4D, 0x65, 0x65, 0x74, 0x69, 0x6E, 0x67,
                        0x46, 0x69, 0x6C, 0x65, 0x0D, 0x0A, 0x4F, 0x72, 0x69, 0x67, 0x69, 0x6E,
                        0x61, 0x6C, 0x20, 0x43, 0x72, 0x65, 0x61, 0x74, 0x6F, 0x72, 0x3A,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
