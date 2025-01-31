use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857887: FileFormat = FileFormat {
    id: 105_857_887,
    puid: "wikidata/105857887",
    name: "Libretro core Information",
    extensions: &["info"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x64, 0x69, 0x73, 0x70, 0x6C, 0x61, 0x79, 0x5F, 0x6E, 0x61, 0x6D, 0x65, 0x20,
                    0x3D, 0x20, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
