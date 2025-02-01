use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866513: FileFormat = FileFormat {
    id: 105_866_513,
    puid: "wikidata/105866513",
    name: "BIS P3D ODOL model",
    extensions: &["p3d"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x44, 0x4F, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
