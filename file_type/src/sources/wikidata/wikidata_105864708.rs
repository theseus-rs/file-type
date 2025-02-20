use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864708: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_708,
        source_type: SourceType::Wikidata,
        name: "Extended TCPDUMP's style capture (little-endian)",
        extensions: &["pcap"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x34, 0xCD, 0xB2, 0xA1])],
                },
            }],
        }],
        related_formats: &[],
    },
};
