use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858226: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_226,
        source_type: SourceType::Wikidata,
        name: "EnCase hash Map (v4)",
        extensions: &["enmap"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x4E, 0x4D, 0x41, 0x50, 0x20, 0x56, 0x34, 0x0B, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
