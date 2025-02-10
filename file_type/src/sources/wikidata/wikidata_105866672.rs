use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866672: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_672,
        source_type: SourceType::Wikidata,
        name: "Pebble Draw Command sequence",
        extensions: &["pdc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x44, 0x43, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
