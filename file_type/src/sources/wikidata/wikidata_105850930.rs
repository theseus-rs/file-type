use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850930: FileFormat = FileFormat {
    id: 105_850_930,
    source_type: SourceType::Wikidata,
    name: "Text - BOCU-1 encoded",
    extensions: &["txt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFB, 0xEE, 0x28])],
            },
        }],
    }],
    related_formats: &[],
};
