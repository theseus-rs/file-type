use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860877: FileFormat = FileFormat {
    id: 105_860_877,
    source_type: SourceType::Wikidata,
    name: "R documentation",
    extensions: &["rd"],
    media_types: &["text/plain", "text/x-r-doc"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5C, 0x6E, 0x61, 0x6D, 0x65, 0x7B])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5C, 0x6E, 0x61, 0x6D, 0x65, 0x7B])],
                },
            }],
        },
    ],
    related_formats: &[],
};
