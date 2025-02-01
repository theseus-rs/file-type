use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862020: FileFormat = FileFormat {
    id: 105_862_020,
    puid: "wikidata/105862020",
    name: "Descent Mission",
    extensions: &["msn"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6E, 0x61, 0x6D, 0x65, 0x20, 0x3D, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
