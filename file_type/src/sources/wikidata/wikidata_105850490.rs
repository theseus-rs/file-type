use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850490: FileFormat = FileFormat {
    id: 105_850_490,
    source_type: SourceType::Wikidata,
    name: "SETool encrypted firmware",
    extensions: &["cry", "ssw"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x72, 0x59, 0x70, 0x54, 0x76])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x72, 0x59, 0x70, 0x54, 0x76])],
                },
            }],
        },
    ],
    related_formats: &[],
};
