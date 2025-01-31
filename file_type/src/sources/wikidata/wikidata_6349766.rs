use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_6349766: FileFormat = FileFormat {
    id: 6_349_766,
    puid: "wikidata/6349766",
    name: "VCDIFF",
    extensions: &["vcdiff"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xD6, 0xC3, 0xC4])],
            },
        }],
    }],
    related_formats: &[],
};
