use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864742: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_742,
        source_type: SourceType::Wikidata,
        name: "Picozu Workspace (v1.0.0)",
        extensions: &["pzw"],
        media_types: &["text/json"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7B, 0x22, 0x76, 0x22, 0x3A, 0x22, 0x31, 0x2E, 0x30, 0x2E, 0x30, 0x22,
                        0x2C, 0x22, 0x74, 0x22, 0x3A, 0x22, 0x50, 0x5A, 0x57, 0x22, 0x2C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
