use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const CUSTOM_3: FileFormat = FileFormat {
    id: 3,
    source_type: SourceType::Custom,
    name: "DuckDB",
    extensions: &["duckdb"],
    media_types: &["application/vnd.duckdb.file"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Any(&[&[
                    Token::WildcardCount(8),
                    Token::Literal(&[0x44, 0x55, 0x43, 0x4B]),
                ]])],
            },
        }],
    }],
    related_formats: &[],
};
