use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861983: FileFormat = FileFormat {
    id: 105_861_983,
    puid: "wikidata/105861983",
    name: "HP ME10 database (ASCII) (with rem)",
    extensions: &["mi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x7E, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
