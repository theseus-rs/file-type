use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853755: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_755,
        source_type: SourceType::Wikidata,
        name: "SQLite Archive compressed",
        extensions: &["sqlar"],
        media_types: &["application/x-sqlite3"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x51, 0x4C, 0x69, 0x74, 0x65, 0x20, 0x66, 0x6F, 0x72, 0x6D, 0x61,
                        0x74, 0x20, 0x33,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
