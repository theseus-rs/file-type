use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854969: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_969,
        source_type: SourceType::Wikidata,
        name: "Arahne loom layout",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x4C, 0x4F, 0x4F, 0x4D, 0x5F, 0x4C, 0x41, 0x59, 0x4F, 0x55, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
