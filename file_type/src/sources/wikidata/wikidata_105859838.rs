use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859838: FileFormat = FileFormat {
    id: 105_859_838,
    puid: "wikidata/105859838",
    name: "FreeMotion Flash movie",
    extensions: &["sqf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x06, 0x00, 0x00, 0x00, 0x47, 0x4D, 0x6F, 0x76, 0x69, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
