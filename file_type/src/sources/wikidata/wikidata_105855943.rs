use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855943: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_943,
        source_type: SourceType::Wikidata,
        name: "Kyle game data container",
        extensions: &["dta"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4B, 0x79, 0x6C, 0x65, 0x20, 0x44, 0x54, 0x41,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
