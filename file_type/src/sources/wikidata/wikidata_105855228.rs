use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855228: FileFormat = FileFormat {
    id: 105_855_228,
    puid: "wikidata/105855228",
    name: "FastDir-like quick directory lookup data",
    extensions: &["fd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x48, 0x43, 0x48])],
            },
        }],
    }],
    related_formats: &[],
};
