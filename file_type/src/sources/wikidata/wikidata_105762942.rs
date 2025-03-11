use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762942: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_942,
        source_type: SourceType::Wikidata,
        name: "Hotbar skin",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x58, 0x49, 0x50, 0x5F, 0x31, 0x2E, 0x30, 0x3B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
