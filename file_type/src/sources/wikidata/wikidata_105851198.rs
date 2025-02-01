use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851198: FileFormat = FileFormat {
    id: 105_851_198,
    puid: "wikidata/105851198",
    name: "Dillo bookmark",
    extensions: &["txt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3A, 0x73, 0x30, 0x3A, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
