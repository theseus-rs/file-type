use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853685: FileFormat = FileFormat {
    id: 105_853_685,
    puid: "wikidata/105853685",
    name: "LZHAM compressed data",
    extensions: &["lzham"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x5A, 0x48, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
