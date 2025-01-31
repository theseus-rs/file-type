use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849642: FileFormat = FileFormat {
    id: 105_849_642,
    puid: "wikidata/105849642",
    name: "Compact compressed data",
    extensions: &["c"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0x1F])],
            },
        }],
    }],
    related_formats: &[],
};
