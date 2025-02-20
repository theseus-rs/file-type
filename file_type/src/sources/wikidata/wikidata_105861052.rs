use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861052: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_052,
        source_type: SourceType::Wikidata,
        name: "EEDraw Library",
        extensions: &["lib"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x45, 0x44, 0x52, 0x41, 0x57, 0x2D, 0x4C, 0x49, 0x42, 0x20, 0x56,
                        0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
