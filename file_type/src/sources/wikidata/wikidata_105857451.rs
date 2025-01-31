use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857451: FileFormat = FileFormat {
    id: 105_857_451,
    puid: "wikidata/105857451",
    name: "Create+Shade 3D scene",
    extensions: &["3d"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x26, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
