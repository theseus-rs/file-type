use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866814: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_814,
        source_type: SourceType::Wikidata,
        name: "Extended TCPDUMP's style capture (big-endian)",
        extensions: &["pcap"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xA1, 0xB2, 0xCD, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
