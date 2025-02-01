use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861736: FileFormat = FileFormat {
    id: 105_861_736,
    puid: "wikidata/105861736",
    name: "Simulink libray",
    extensions: &["mdl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x69, 0x62, 0x72, 0x61, 0x72, 0x79, 0x20, 0x7B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
