use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851075: FileFormat = FileFormat {
    id: 105_851_075,
    puid: "wikidata/105851075",
    name: "Torque skeleton asset (XML)",
    extensions: &["taml"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x53, 0x6B, 0x65, 0x6C, 0x65, 0x74, 0x6F, 0x6E, 0x41, 0x73, 0x73, 0x65,
                    0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
