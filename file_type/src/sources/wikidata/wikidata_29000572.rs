use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000572: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_572,
        source_type: SourceType::Wikidata,
        name: "SmartSniff Packet",
        extensions: &["ssp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x4D, 0x53, 0x4E, 0x46, 0x32, 0x30, 0x30, 0x04, 0x00, 0xC0, 0xA8,
                        0x7B, 0xC8, 0x18, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
