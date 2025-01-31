use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851457: FileFormat = FileFormat {
    id: 105_851_457,
    puid: "wikidata/105851457",
    name: "Text - UTF-7 encoded",
    extensions: &["txt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2B, 0x2F, 0x76])],
            },
        }],
    }],
    related_formats: &[],
};
