use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859184: FileFormat = FileFormat {
    id: 105_859_184,
    puid: "wikidata/105859184",
    name: "Nintendo GameCube/Wii 3D Model (ASCII)",
    extensions: &["bdl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x33, 0x44, 0x32, 0x62, 0x64, 0x6C])],
            },
        }],
    }],
    related_formats: &[],
};
