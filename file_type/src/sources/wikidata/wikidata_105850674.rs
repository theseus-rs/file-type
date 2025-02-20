use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850674: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_674,
        source_type: SourceType::Wikidata,
        name: "ksudoku puzzle",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45, 0x20, 0x6B, 0x73,
                        0x75, 0x64, 0x6F, 0x6B, 0x75, 0x2D, 0x67, 0x72, 0x61, 0x70, 0x68, 0x3E,
                        0x0A, 0x3C, 0x6B, 0x73, 0x75, 0x64, 0x6F, 0x6B, 0x75, 0x2D, 0x67, 0x72,
                        0x61, 0x70, 0x68,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
