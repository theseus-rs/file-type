use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_109302929: FileType = FileType {
    file_format: &FileFormat {
        id: 109_302_929,
        source_type: SourceType::Wikidata,
        name: "Asymetrix Compel Presentation, version 2",
        extensions: &["cpl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x03, 0x4A, 0x42, 0x4F, 0x4F, 0xF5, 0x3C, 0x55,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
