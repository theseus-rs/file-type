use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853274: FileFormat = FileFormat {
    id: 105_853_274,
    puid: "wikidata/105853274",
    name: "SynWrite Snippet",
    extensions: &["synw-snippet"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6E, 0x61, 0x6D, 0x65, 0x3D])],
            },
        }],
    }],
    related_formats: &[],
};
