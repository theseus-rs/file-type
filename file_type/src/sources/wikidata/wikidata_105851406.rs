use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851406: FileFormat = FileFormat {
    id: 105_851_406,
    puid: "wikidata/105851406",
    name: "Torque animation asset (XML)",
    extensions: &["taml"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x41, 0x6E, 0x69, 0x6D, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x41, 0x73, 0x73,
                    0x65, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
