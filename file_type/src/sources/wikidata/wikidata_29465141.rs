use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29465141: FileFormat = FileFormat {
    id: 29_465_141,
    puid: "wikidata/29465141",
    name: "Valve Map Format",
    extensions: &["vmf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x69, 0x6E, 0x66, 0x6F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
