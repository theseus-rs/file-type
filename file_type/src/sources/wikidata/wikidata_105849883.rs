use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849883: FileFormat = FileFormat {
    id: 105_849_883,
    source_type: SourceType::Wikidata,
    name: "Classical Text Editor document (v8)",
    extensions: &["cte"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x43, 0x54, 0x45, 0x5F, 0x38, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
