use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851552: FileFormat = FileFormat {
    id: 105_851_552,
    source_type: SourceType::Wikidata,
    name: "Text Plus document (v3.0)",
    extensions: &["txp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x50, 0x33, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
