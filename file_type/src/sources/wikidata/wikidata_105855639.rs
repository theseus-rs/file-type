use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855639: FileFormat = FileFormat {
    id: 105_855_639,
    source_type: SourceType::Wikidata,
    name: "Wireshark traffic log",
    extensions: &["out", "txt"],
    media_types: &[],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x72, 0x61, 0x6D, 0x65, 0x20])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x72, 0x61, 0x6D, 0x65, 0x20])],
                },
            }],
        },
    ],
    related_formats: &[],
};
