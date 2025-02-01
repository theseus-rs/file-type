use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855200: FileFormat = FileFormat {
    id: 105_855_200,
    puid: "wikidata/105855200",
    name: "PreForm 3D object container",
    extensions: &["form"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xF0, 0x0D, 0xF0, 0x0D, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00,
                    0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
