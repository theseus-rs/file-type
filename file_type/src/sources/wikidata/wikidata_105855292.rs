use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855292: FileFormat = FileFormat {
    id: 105_855_292,
    puid: "wikidata/105855292",
    name: "Finalburn Alpha movie capture",
    extensions: &["fr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x42, 0x31, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
