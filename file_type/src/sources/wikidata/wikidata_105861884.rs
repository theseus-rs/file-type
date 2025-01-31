use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861884: FileFormat = FileFormat {
    id: 105_861_884,
    puid: "wikidata/105861884",
    name: "Mythroad Platform application",
    extensions: &["mrp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x52, 0x50, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
