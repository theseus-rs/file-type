use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861747: FileFormat = FileFormat {
    id: 105_861_747,
    puid: "wikidata/105861747",
    name: "Exotic AdLib module",
    extensions: &["xad"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x41, 0x44, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
