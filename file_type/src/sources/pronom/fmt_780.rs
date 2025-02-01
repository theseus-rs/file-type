use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_780: FileFormat = FileFormat {
    id: 1_579,
    puid: "fmt/780",
    name: "pcap Next Generation Packet Capture",
    extensions: &["pcapng"],
    media_types: &["application/vnd.tcpdump.pcap"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x0A, 0x0D, 0x0D, 0x0A]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x1A, 0x2B, 0x3C, 0x4D]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x0A, 0x0D, 0x0D, 0x0A]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x4D, 0x3C, 0x2B, 0x1A]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[],
};
