use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_133271766: FileType = FileType {
    file_format: &FileFormat {
        id: 133_271_766,
        source_type: SourceType::Wikidata,
        name: "DuckDB database file",
        extensions: &["ddb", "duckdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x55, 0x43, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
