use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856822: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_822,
        source_type: SourceType::Wikidata,
        name: "Google Drive link (generic)",
        extensions: &[],
        media_types: &["text/json"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7B, 0x22, 0x75, 0x72, 0x6C, 0x22, 0x3A, 0x20, 0x22, 0x68, 0x74, 0x74,
                        0x70, 0x73, 0x3A, 0x2F, 0x2F, 0x64, 0x6F, 0x63, 0x73, 0x2E, 0x67, 0x6F,
                        0x6F, 0x67, 0x6C, 0x65, 0x2E, 0x63, 0x6F, 0x6D, 0x2F, 0x6F, 0x70, 0x65,
                        0x6E, 0x3F, 0x69, 0x64, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
