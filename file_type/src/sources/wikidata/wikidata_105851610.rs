use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851610: FileFormat = FileFormat {
    id: 105_851_610,
    puid: "wikidata/105851610",
    name: "Torque module definition (XML)",
    extensions: &["taml"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x4D, 0x6F, 0x64, 0x75, 0x6C, 0x65, 0x44, 0x65, 0x66, 0x69, 0x6E, 0x69,
                    0x74, 0x69, 0x6F, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
