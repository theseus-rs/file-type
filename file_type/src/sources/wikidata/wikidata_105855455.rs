use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855455: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_455,
        source_type: SourceType::Wikidata,
        name: "Fanuc parameters file",
        extensions: &["mem", "st1h"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x41, 0x4E, 0x55, 0x43, 0x52, 0x4F, 0x4D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
