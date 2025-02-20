use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851380: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_380,
        source_type: SourceType::Wikidata,
        name: "Xoom Tutor tutorial (with title)",
        extensions: &["tut"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x74, 0x69, 0x74, 0x6C, 0x65, 0x20, 0x58, 0x6F, 0x6F, 0x6D, 0x54, 0x75,
                        0x74, 0x6F, 0x72, 0x3A, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
