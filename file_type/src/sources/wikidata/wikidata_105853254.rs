use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853254: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_254,
        source_type: SourceType::Wikidata,
        name: "SQLite rollbak journal",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xD9, 0xD5, 0x05, 0xF9, 0x20, 0xA1, 0x63, 0xD7,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
