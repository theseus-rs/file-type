use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000498: FileFormat = FileFormat {
    id: 29_000_498,
    puid: "wikidata/29000498",
    name: "OrCAD library",
    extensions: &["lib"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2A, 0x20, 0x50, 0x53, 0x70, 0x69, 0x63, 0x65, 0x20, 0x4D, 0x6F, 0x64, 0x65,
                    0x6C, 0x20, 0x45, 0x64, 0x69, 0x74, 0x6F, 0x72, 0x20, 0x2D, 0x20, 0x56, 0x65,
                    0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
