use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859290: FileFormat = FileFormat {
    id: 105_859_290,
    puid: "wikidata/105859290",
    name: "Balance of Power: The 1990 Ed. saved game",
    extensions: &["bop"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x4F, 0x50, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
