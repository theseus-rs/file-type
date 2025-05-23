use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864860: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_860,
        source_type: SourceType::Wikidata,
        name: "Adobe Premiere Title (BE)",
        extensions: &["prtl"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFE, 0xFF, 0x00, 0x3C, 0x00, 0x3F, 0x00, 0x78, 0x00, 0x6D, 0x00, 0x6C,
                        0x00, 0x20, 0x00, 0x76, 0x00, 0x65, 0x00, 0x72, 0x00, 0x73, 0x00, 0x69,
                        0x00, 0x6F, 0x00, 0x6E, 0x00, 0x3D, 0x00, 0x22, 0x00, 0x31, 0x00, 0x2E,
                        0x00, 0x30, 0x00, 0x22, 0x00, 0x20, 0x00, 0x65, 0x00, 0x6E, 0x00, 0x63,
                        0x00, 0x6F, 0x00, 0x64, 0x00, 0x69, 0x00, 0x6E, 0x00, 0x67, 0x00, 0x3D,
                        0x00, 0x22, 0x00, 0x55, 0x00, 0x54, 0x00, 0x46, 0x00, 0x2D, 0x00, 0x31,
                        0x00, 0x36, 0x00, 0x22, 0x00, 0x20, 0x00, 0x3F, 0x00, 0x3E, 0x00, 0x3C,
                        0x00, 0x41, 0x00, 0x64, 0x00, 0x6F, 0x00, 0x62, 0x00, 0x65, 0x00, 0x5F,
                        0x00, 0x52, 0x00, 0x6F, 0x00, 0x6F, 0x00, 0x74, 0x00, 0x3E, 0x00, 0x3C,
                        0x00, 0x41, 0x00, 0x64, 0x00, 0x6F, 0x00, 0x62, 0x00, 0x65, 0x00, 0x5F,
                        0x00, 0x54, 0x00, 0x69, 0x00, 0x74, 0x00, 0x6C, 0x00, 0x65, 0x00, 0x3E,
                        0x00, 0x3C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
