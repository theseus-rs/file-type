use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850667: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_667,
        source_type: SourceType::Wikidata,
        name: "BeebEm Keymap",
        extensions: &["kmap"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2A, 0x2A, 0x2A, 0x20, 0x42, 0x65, 0x65, 0x62, 0x45, 0x6D, 0x20, 0x4B,
                        0x65, 0x79, 0x6D, 0x61, 0x70, 0x20, 0x2A, 0x2A, 0x2A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
