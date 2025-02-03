use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849642: FileFormat = FileFormat {
    id: 105_849_642,
    source_type: SourceType::Wikidata,
    name: "Compact compressed data",
    extensions: &["c"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0x1F])],
            },
        }],
    }],
    related_formats: &[],
};
