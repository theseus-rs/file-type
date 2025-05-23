use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856428: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_428,
        source_type: SourceType::Wikidata,
        name: "Windows Media Player sync info",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFE, 0x3C, 0x00, 0x44, 0x00, 0x65, 0x00, 0x76, 0x00, 0x69, 0x00,
                        0x63, 0x00, 0x65, 0x00, 0x49, 0x00, 0x6E, 0x00, 0x66, 0x00, 0x6F, 0x00,
                        0x3E, 0x00, 0x0D, 0x00, 0x0A, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00,
                        0x20, 0x00, 0x3C, 0x00, 0x57, 0x00, 0x4D, 0x00, 0x50, 0x00, 0x20, 0x00,
                        0x44, 0x00, 0x65, 0x00, 0x76, 0x00, 0x69, 0x00, 0x63, 0x00, 0x65, 0x00,
                        0x49, 0x00, 0x44, 0x00, 0x3D, 0x00, 0x22, 0x00, 0x7B, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
