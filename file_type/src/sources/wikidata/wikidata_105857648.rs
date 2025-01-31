use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857648: FileFormat = FileFormat {
    id: 105_857_648,
    puid: "wikidata/105857648",
    name: "Ensoniq TS-10 EDM disk image (HD)",
    extensions: &["edt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0D, 0x0A, 0x54, 0x53, 0x2D, 0x31, 0x30, 0x20, 0x28, 0x48, 0x44, 0x29, 0x20,
                    0x44, 0x69, 0x73, 0x6B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
