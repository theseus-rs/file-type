use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857263: FileFormat = FileFormat {
    id: 105_857_263,
    puid: "wikidata/105857263",
    name: "KeyShot environment",
    extensions: &["hdz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x68, 0x64, 0x7A, 0x6C, 0x75, 0x78])],
            },
        }],
    }],
    related_formats: &[],
};
