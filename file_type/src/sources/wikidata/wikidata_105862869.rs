use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862869: FileFormat = FileFormat {
    id: 105_862_869,
    puid: "wikidata/105862869",
    name: "Simulink Model",
    extensions: &["mdl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x6F, 0x64, 0x65, 0x6C, 0x20, 0x7B])],
            },
        }],
    }],
    related_formats: &[],
};
