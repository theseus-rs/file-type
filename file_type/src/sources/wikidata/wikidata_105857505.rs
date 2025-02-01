use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857505: FileFormat = FileFormat {
    id: 105_857_505,
    puid: "wikidata/105857505",
    name: "3D World Studio model",
    extensions: &["3dw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x44, 0x57, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
