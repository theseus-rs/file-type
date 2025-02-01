use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862964: FileFormat = FileFormat {
    id: 105_862_964,
    puid: "wikidata/105862964",
    name: "Faust Music Creator module",
    extensions: &["sng"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x4D, 0x43, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
