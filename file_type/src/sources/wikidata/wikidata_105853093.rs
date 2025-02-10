use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853093: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_093,
        source_type: SourceType::Wikidata,
        name: "Scala Multimedia Script (v3.0)",
        extensions: &["script"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x33, 0x2E, 0x30, 0x0A, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
