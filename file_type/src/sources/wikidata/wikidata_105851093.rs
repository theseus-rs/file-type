use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851093: FileFormat = FileFormat {
    id: 105_851_093,
    puid: "wikidata/105851093",
    name: "TurboCAD drawing (v3 Win)",
    extensions: &["tcw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
