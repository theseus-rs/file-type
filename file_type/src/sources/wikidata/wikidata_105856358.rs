use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856358: FileFormat = FileFormat {
    id: 105_856_358,
    source_type: SourceType::Wikidata,
    name: "DB/TextWorks Database",
    extensions: &["dbr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x42, 0x52, 0x20, 0x30, 0x30, 0x31, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
