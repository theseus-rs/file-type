use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2876668: FileFormat = FileFormat {
    id: 2_876_668,
    puid: "wikidata/2876668",
    name: "B3D",
    extensions: &["b3d"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x42, 0x33, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
