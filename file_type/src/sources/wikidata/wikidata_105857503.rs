use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857503: FileFormat = FileFormat {
    id: 105_857_503,
    puid: "wikidata/105857503",
    name: "QuickDraw 3D Metafile (text)",
    extensions: &["3dmf"],
    media_types: &["x-world/x-3dmf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x33, 0x44, 0x4D, 0x65, 0x74, 0x61, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x28, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
