use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851173: FileFormat = FileFormat {
    id: 105_851_173,
    source_type: SourceType::Wikidata,
    name: "TASM instructions definition Table",
    extensions: &["tab"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x22, 0x54, 0x41, 0x53, 0x4D, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
