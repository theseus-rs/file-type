use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856367: FileFormat = FileFormat {
    id: 105_856_367,
    puid: "wikidata/105856367",
    name: "Normality game data archive",
    extensions: &["das"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x41, 0x53, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
