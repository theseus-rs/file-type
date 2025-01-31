use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862840: FileFormat = FileFormat {
    id: 105_862_840,
    puid: "wikidata/105862840",
    name: "Musik-Trainer Notation",
    extensions: &["mtn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x55, 0x53, 0x49, 0x43, 0x20, 0x54, 0x52, 0x41, 0x49, 0x4E, 0x45, 0x52,
                    0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
