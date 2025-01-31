use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851402: FileFormat = FileFormat {
    id: 105_851_402,
    puid: "wikidata/105851402",
    name: "Torque image asset (XML)",
    extensions: &["taml"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x41, 0x73, 0x73, 0x65, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
