use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861091: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_091,
        source_type: SourceType::Wikidata,
        name: "Lenex XML data",
        extensions: &["lef"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x20, 0x65, 0x6E, 0x63, 0x6F,
                        0x64, 0x69, 0x6E, 0x67, 0x3D, 0x22, 0x49, 0x53, 0x4F, 0x2D, 0x38, 0x38,
                        0x35, 0x39, 0x2D, 0x31, 0x22, 0x3F, 0x3E, 0x0D, 0x0A, 0x3C, 0x4C, 0x45,
                        0x4E, 0x45, 0x58, 0x3E, 0x0D, 0x0A, 0x20, 0x3C, 0x56, 0x45, 0x52, 0x53,
                        0x49, 0x4F, 0x4E, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
