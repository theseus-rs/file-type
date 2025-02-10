use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856329: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_329,
        source_type: SourceType::Wikidata,
        name: "Delphi Diagram Portfolio",
        extensions: &["ddp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x07, 0x18, 0x44, 0x45, 0x4C, 0x50, 0x48, 0x49, 0x2E, 0x44, 0x49, 0x41,
                        0x47, 0x52, 0x41, 0x4D, 0x2E, 0x50, 0x4F, 0x52, 0x54, 0x46, 0x4F, 0x4C,
                        0x49, 0x4F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
