use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857891: FileFormat = FileFormat {
    id: 105_857_891,
    puid: "wikidata/105857891",
    name: "Ensoniq VFX-SD EDM disk image",
    extensions: &["edv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0D, 0x0A, 0x56, 0x46, 0x58, 0x2D, 0x53, 0x44, 0x20, 0x44, 0x69, 0x73, 0x6B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
