use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851316: FileFormat = FileFormat {
    id: 105_851_316,
    source_type: SourceType::Wikidata,
    name: "Xcode Text Based Definition",
    extensions: &["tbd"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2D, 0x2D, 0x2D])],
            },
        }],
    }],
    related_formats: &[],
};
