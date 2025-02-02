use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858509: FileFormat = FileFormat {
    id: 105_858_509,
    source_type: SourceType::Wikidata,
    name: "MGR bitmap (old, 1-bit, 32-bit aligned)",
    extensions: &["mgr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x78, 0x7A])],
            },
        }],
    }],
    related_formats: &[],
};
