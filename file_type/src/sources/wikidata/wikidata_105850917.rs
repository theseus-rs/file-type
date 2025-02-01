use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850917: FileFormat = FileFormat {
    id: 105_850_917,
    puid: "wikidata/105850917",
    name: "TurboCalc Document",
    extensions: &["tcd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0x00, 0x0C, 0x54, 0x55, 0x52, 0x42, 0x4F, 0x43, 0x41, 0x4C, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
