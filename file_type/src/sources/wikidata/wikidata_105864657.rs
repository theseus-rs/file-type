use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864657: FileFormat = FileFormat {
    id: 105_864_657,
    puid: "wikidata/105864657",
    name: "Aegis Animator Polygon",
    extensions: &["poly"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2A, 0x61, 0x63, 0x74, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
