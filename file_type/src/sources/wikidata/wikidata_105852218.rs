use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852218: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_218,
        source_type: SourceType::Wikidata,
        name: "Duxbury Scrub Table",
        extensions: &["sbt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7C, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
