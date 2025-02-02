use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const CUSTOM_3: FileFormat = FileFormat {
    id: 3,
    source_type: SourceType::Custom,
    name: "DuckDB",
    extensions: &["duckdb"],
    media_types: &["application/vnd.duckdb.file"],
    internal_signatures: &[InternalSignature {
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
