use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854714: FileFormat = FileFormat {
    id: 105_854_714,
    puid: "wikidata/105854714",
    name: "bsc compressed data",
    extensions: &["bsc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x62, 0x73, 0x63, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
