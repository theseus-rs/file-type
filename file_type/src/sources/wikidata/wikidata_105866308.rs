use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866308: FileFormat = FileFormat {
    id: 105_866_308,
    puid: "wikidata/105866308",
    name: "BIS P3D MLOD model",
    extensions: &["p3d"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4C, 0x4F, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
