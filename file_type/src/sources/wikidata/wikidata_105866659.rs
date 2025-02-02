use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866659: FileFormat = FileFormat {
    id: 105_866_659,
    source_type: SourceType::Wikidata,
    name: "Lightwork Pattern (name first)",
    extensions: &["pattern"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6E, 0x61, 0x6D, 0x65, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
