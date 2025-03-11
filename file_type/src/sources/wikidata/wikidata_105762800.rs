use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762800: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_800,
        source_type: SourceType::Wikidata,
        name: "XNA Game Studio resource",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x44, 0x41, 0x54, 0x41, 0x2E, 0x52, 0x45, 0x53, 0x4F, 0x55, 0x52,
                        0x43, 0x45, 0x5D, 0x00, 0x00, 0x00, 0x28, 0x47, 0x41, 0x4D, 0x45, 0x5F,
                        0x53, 0x54, 0x55, 0x44, 0x49, 0x4F, 0x5F, 0x52, 0x45, 0x53, 0x4F, 0x55,
                        0x52, 0x43, 0x45, 0x5F, 0x44, 0x41, 0x54, 0x41, 0x5F, 0x46, 0x49, 0x4C,
                        0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
