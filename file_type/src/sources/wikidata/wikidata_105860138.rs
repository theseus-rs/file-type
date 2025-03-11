use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860138: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_138,
        source_type: SourceType::Wikidata,
        name: "Legacy RSA1 keys format",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x53, 0x48, 0x20, 0x50, 0x52, 0x49, 0x56, 0x41, 0x54, 0x45, 0x20,
                        0x4B, 0x45, 0x59, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x20, 0x46, 0x4F, 0x52,
                        0x4D, 0x41, 0x54, 0x20, 0x31, 0x2E, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
