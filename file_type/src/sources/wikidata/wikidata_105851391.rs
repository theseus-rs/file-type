use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851391: FileFormat = FileFormat {
    id: 105_851_391,
    puid: "wikidata/105851391",
    name: "Torque sprite asset (XML)",
    extensions: &["taml"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x53, 0x70, 0x72, 0x69, 0x74, 0x65])],
            },
        }],
    }],
    related_formats: &[],
};
