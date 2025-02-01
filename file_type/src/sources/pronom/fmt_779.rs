use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_779: FileFormat = FileFormat {
    id: 1_578,
    puid: "fmt/779",
    name: "pcap Packet Capture",
    extensions: &["pcap", "cap", "dmp"],
    media_types: &["application/vnd.tcpdump.pcap"],
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
                    tokens: &[Token::Literal(&[0xD4, 0xC3, 0xB2, 0xA1])],
                },
            }],
        },
    ],
    related_formats: &[],
};
