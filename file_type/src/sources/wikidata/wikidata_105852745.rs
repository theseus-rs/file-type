use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852745: FileFormat = FileFormat {
    id: 105_852_745,
    puid: "wikidata/105852745",
    name: "E-Z FM Synthesizer sound Settings",
    extensions: &["set"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x4F, 0x44, 0x44, 0x77, 0x61, 0x73, 0x48, 0x45, 0x52, 0x45, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
