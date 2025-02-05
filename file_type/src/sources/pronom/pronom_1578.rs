use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1578: FileFormat = FileFormat {
    id: 1_578,
    source_type: SourceType::Pronom,
    name: "pcap Packet Capture",
    extensions: &["pcap", "cap", "dmp"],
    media_types: &["application/vnd.tcpdump.pcap"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xA1, 0xB2, 0xC3, 0xD4])],
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
