use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854100: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_100,
        source_type: SourceType::Wikidata,
        name: "TCPDUMP's style capture (big-endian)",
        extensions: &["acp", "pcap"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xA1, 0xB2, 0xC3, 0xD4])],
                },
            }],
        }],
        related_formats: &[],
    },
};
