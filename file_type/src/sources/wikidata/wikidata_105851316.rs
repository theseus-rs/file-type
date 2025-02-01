use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851316: FileFormat = FileFormat {
    id: 105_851_316,
    puid: "wikidata/105851316",
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
