use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865016: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_016,
        source_type: SourceType::Wikidata,
        name: "Portable Bridge Notation (v2.0)",
        extensions: &["pbn"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x25, 0x20, 0x50, 0x42, 0x4E, 0x20, 0x32, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
