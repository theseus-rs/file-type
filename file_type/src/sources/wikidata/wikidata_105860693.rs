use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860693: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_693,
        source_type: SourceType::Wikidata,
        name: "Grand Theft Auto: San Andreas replay",
        extensions: &["rep"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x74, 0x61, 0x53, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
