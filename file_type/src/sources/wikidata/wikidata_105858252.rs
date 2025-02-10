use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858252: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_252,
        source_type: SourceType::Wikidata,
        name: "EmEditor Syntax file",
        extensions: &["esy"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFE, 0x3B, 0x00, 0x20, 0x00, 0x45, 0x00, 0x6D, 0x00, 0x45, 0x00,
                        0x64, 0x00, 0x69, 0x00, 0x74, 0x00, 0x6F, 0x00, 0x72, 0x00, 0x20, 0x00,
                        0x53, 0x00, 0x79, 0x00, 0x6E, 0x00, 0x74, 0x00, 0x61, 0x00, 0x78, 0x00,
                        0x20, 0x00, 0x46, 0x00, 0x69, 0x00, 0x6C, 0x00, 0x65, 0x00, 0x0D, 0x00,
                        0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
