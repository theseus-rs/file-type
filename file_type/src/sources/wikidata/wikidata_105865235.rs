use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865235: FileFormat = FileFormat {
    id: 105_865_235,
    puid: "wikidata/105865235",
    name: "PIMPLE compressed data (v1)",
    extensions: &["pim"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x50, 0x49, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
