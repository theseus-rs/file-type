use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859554: FileFormat = FileFormat {
    id: 105_859_554,
    source_type: SourceType::Wikidata,
    name: "CRYO UBB video",
    extensions: &["hnm", "ubb"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x42, 0x42, 0x32])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x42, 0x42, 0x32])],
                },
            }],
        },
    ],
    related_formats: &[],
};
