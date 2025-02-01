use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850915: FileFormat = FileFormat {
    id: 105_850_915,
    puid: "wikidata/105850915",
    name: "Text - SCSU encoded",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0E, 0xFE, 0xFF])],
            },
        }],
    }],
    related_formats: &[],
};
