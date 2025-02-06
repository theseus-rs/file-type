use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854060: FileFormat = FileFormat {
    id: 105_854_060,
    source_type: SourceType::Wikidata,
    name: "TCPDUMP's style capture (little-endian)",
    extensions: &["acp", "pcap"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xD4, 0xC3, 0xB2, 0xA1])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xD4, 0xC3, 0xB2, 0xA1])],
                },
            }],
        },
    ],
    related_formats: &[],
};
