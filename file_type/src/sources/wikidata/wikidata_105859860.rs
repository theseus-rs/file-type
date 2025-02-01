use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859860: FileFormat = FileFormat {
    id: 105_859_860,
    puid: "wikidata/105859860",
    name: "8088 Corruption TMV video",
    extensions: &["tmv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x4D, 0x41, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
