use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860873: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_873,
        source_type: SourceType::Wikidata,
        name: "SpaceCAD rocket model",
        extensions: &["roc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3B, 0x0D, 0x3B, 0x20, 0x53, 0x70, 0x61, 0x63, 0x65, 0x43, 0x41, 0x44,
                        0x28, 0x74, 0x6D, 0x29, 0x20, 0x4D, 0x6F, 0x64, 0x65, 0x6C, 0x20, 0x52,
                        0x6F, 0x63, 0x6B, 0x65, 0x74, 0x20, 0x53, 0x6F, 0x66, 0x74, 0x77, 0x61,
                        0x72, 0x65, 0x0D, 0x3B, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                        0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
