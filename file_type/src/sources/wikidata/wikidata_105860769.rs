use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860769: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_769,
        source_type: SourceType::Wikidata,
        name: "Recolored project",
        extensions: &["rcl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x43, 0x4C, 0x46, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
                        0x42, 0x4D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
