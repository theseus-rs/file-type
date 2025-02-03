use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855897: FileFormat = FileFormat {
    id: 105_855_897,
    source_type: SourceType::Wikidata,
    name: "DB/TextWorks Database Textbase Structure file",
    extensions: &["dbs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
};
