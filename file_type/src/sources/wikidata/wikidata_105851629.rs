use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851629: FileFormat = FileFormat {
    id: 105_851_629,
    puid: "wikidata/105851629",
    name: "Text - UTF-16 (BE) encoded",
    extensions: &["txt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFE, 0xFF])],
            },
        }],
    }],
    related_formats: &[],
};
