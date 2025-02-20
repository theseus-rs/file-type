use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860405: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_405,
        source_type: SourceType::Wikidata,
        name: "OHRRPGCE game",
        extensions: &["rpg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x52, 0x43, 0x48, 0x49, 0x4E, 0x59, 0x4D, 0x2E, 0x4C, 0x4D, 0x50,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
