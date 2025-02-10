use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859371: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_371,
        source_type: SourceType::Wikidata,
        name: "dBASE IV Query",
        extensions: &["qbe"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2A, 0x20, 0x64, 0x42, 0x41, 0x53, 0x45, 0x20, 0x49, 0x56, 0x20, 0x2E,
                        0x51, 0x42, 0x45, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
