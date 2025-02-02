use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850924: FileFormat = FileFormat {
    id: 105_850_924,
    source_type: SourceType::Wikidata,
    name: "Text - UTF-32 (BE) encoded",
    extensions: &["txt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0xFE, 0xFF])],
            },
        }],
    }],
    related_formats: &[],
};
