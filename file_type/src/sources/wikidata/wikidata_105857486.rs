use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857486: FileFormat = FileFormat {
    id: 105_857_486,
    puid: "wikidata/105857486",
    name: "CAD-3D object",
    extensions: &["3d"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3D, 0x3D, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
