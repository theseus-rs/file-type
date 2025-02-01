use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861039: FileFormat = FileFormat {
    id: 105_861_039,
    puid: "wikidata/105861039",
    name: "Sony PS3 LZRC compressed data",
    extensions: &["lzrc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x32, 0x52, 0x4C, 0x5A, 0x06])],
            },
        }],
    }],
    related_formats: &[],
};
