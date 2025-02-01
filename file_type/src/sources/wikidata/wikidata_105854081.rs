use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854081: FileFormat = FileFormat {
    id: 105_854_081,
    puid: "wikidata/105854081",
    name: "Yamaha TX-16W samples audio",
    extensions: &["txw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x4D, 0x38, 0x39, 0x35, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
