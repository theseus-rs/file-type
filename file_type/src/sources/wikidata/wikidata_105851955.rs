use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851955: FileFormat = FileFormat {
    id: 105_851_955,
    source_type: SourceType::Wikidata,
    name: "Snzip compressed (comment-43 format)",
    extensions: &["snappy"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0x06, 0x00, 0x73, 0x6E, 0x61, 0x70, 0x70, 0x79,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
