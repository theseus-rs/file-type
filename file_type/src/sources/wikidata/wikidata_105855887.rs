use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855887: FileFormat = FileFormat {
    id: 105_855_887,
    puid: "wikidata/105855887",
    name: "AutoCAD Drawing eXchange Format (var.1/U)",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x20, 0x30, 0x0A, 0x53, 0x45, 0x43, 0x54, 0x49, 0x4F, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
