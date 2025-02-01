use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859933: FileFormat = FileFormat {
    id: 105_859_933,
    puid: "wikidata/105859933",
    name: "Volition Package game archive data",
    extensions: &["vp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x50, 0x56, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
