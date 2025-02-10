use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860345: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_345,
        source_type: SourceType::Wikidata,
        name: "Rexx-Adventure Saved game",
        extensions: &["ras"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x69, 0x74, 0x6C, 0x65, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
