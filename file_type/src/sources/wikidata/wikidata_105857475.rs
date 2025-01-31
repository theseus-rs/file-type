use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857475: FileFormat = FileFormat {
    id: 105_857_475,
    puid: "wikidata/105857475",
    name: "3D Slash model",
    extensions: &["3dslash"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x33, 0x44, 0x20, 0x53, 0x4C, 0x41, 0x53, 0x48, 0x30, 0x30, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
