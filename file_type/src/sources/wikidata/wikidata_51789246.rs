use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_51789246: FileType = FileType {
    file_format: &FileFormat {
        id: 51_789_246,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Named Plot Style Table",
        extensions: &["stb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x49, 0x41, 0x46, 0x49, 0x4C, 0x45, 0x56, 0x45, 0x52, 0x53, 0x49,
                        0x4F, 0x4E, 0x5F, 0x32, 0x2E, 0x30, 0x2C, 0x53, 0x54, 0x42, 0x56, 0x45,
                        0x52, 0x31, 0x2C, 0x63, 0x6F, 0x6D, 0x70, 0x72, 0x65, 0x73, 0x73, 0x0D,
                        0x0A, 0x70, 0x6D, 0x7A, 0x6C, 0x69, 0x62, 0x63, 0x6F, 0x64, 0x65, 0x63,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
