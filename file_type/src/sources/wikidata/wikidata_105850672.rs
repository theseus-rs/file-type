use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850672: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_672,
        source_type: SourceType::Wikidata,
        name: "Kaspersky Anti-Virus quarantined",
        extensions: &["klq"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4B, 0x4C, 0x51, 0x42, 0x01, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
