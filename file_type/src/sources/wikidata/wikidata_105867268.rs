use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867268: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_268,
        source_type: SourceType::Wikidata,
        name: "NDF National Land Archive Processing System Data Format",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x44, 0x46, 0x5F, 0x52, 0x45, 0x56, 0x49, 0x53, 0x49, 0x4F, 0x4E,
                        0x3D, 0x32, 0x2E, 0x30, 0x30, 0x3B, 0x0A, 0x44, 0x41, 0x54, 0x41, 0x5F,
                        0x53, 0x45, 0x54, 0x5F, 0x54, 0x59, 0x50, 0x45, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
