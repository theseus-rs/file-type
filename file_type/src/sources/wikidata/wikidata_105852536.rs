use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852536: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_536,
        source_type: SourceType::Wikidata,
        name: "Simis format (uncompressed, UTF16-LE)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFE, 0x53, 0x00, 0x49, 0x00, 0x4D, 0x00, 0x49, 0x00, 0x53, 0x00,
                        0x41, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0x00,
                        0x40, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
