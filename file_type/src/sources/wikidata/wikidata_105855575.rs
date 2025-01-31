use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855575: FileFormat = FileFormat {
    id: 105_855_575,
    puid: "wikidata/105855575",
    name: "Oberon V4 Symbol data",
    extensions: &["sym"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xF7, 0x16])],
            },
        }],
    }],
    related_formats: &[],
};
