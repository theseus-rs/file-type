use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762766: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_766,
        source_type: SourceType::Wikidata,
        name: "Monarch Pro model",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x6D, 0x6F, 0x64, 0x65, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69,
                        0x6F, 0x6E, 0x3D, 0x22, 0x58, 0x4D, 0x4C, 0x4D, 0x4F, 0x44,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
