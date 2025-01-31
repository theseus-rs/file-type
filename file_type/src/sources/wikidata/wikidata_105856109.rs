use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856109: FileFormat = FileFormat {
    id: 105_856_109,
    puid: "wikidata/105856109",
    name: "3D Dgf model",
    extensions: &["dgf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x44, 0x47, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
