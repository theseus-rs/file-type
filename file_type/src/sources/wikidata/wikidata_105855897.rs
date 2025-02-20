use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855897: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_897,
        source_type: SourceType::Wikidata,
        name: "DB/TextWorks Database Textbase Structure file",
        extensions: &["dbs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x42, 0x53, 0x20, 0x30, 0x30, 0x39, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
