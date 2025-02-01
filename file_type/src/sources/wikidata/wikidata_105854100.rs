use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854100: FileFormat = FileFormat {
    id: 105_854_100,
    puid: "wikidata/105854100",
    name: "TCPDUMP's style capture (big-endian)",
    extensions: &["acp", "pcap"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xA1, 0xB2, 0xC3, 0xD4])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xA1, 0xB2, 0xC3, 0xD4])],
                },
            }],
        },
    ],
    related_formats: &[],
};
