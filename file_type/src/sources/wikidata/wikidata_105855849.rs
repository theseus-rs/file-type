use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855849: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_849,
        source_type: SourceType::Wikidata,
        name: "Dynamic Env. phonebook",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x59, 0x4E, 0x41, 0x4D, 0x49, 0x43, 0x20, 0x50, 0x48, 0x4F, 0x4E,
                        0x45, 0x0D, 0x0A, 0x3E, 0x0D, 0x0A, 0x3A, 0x4E, 0x61, 0x6D, 0x65, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
