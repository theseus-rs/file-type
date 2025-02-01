use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854261: FileFormat = FileFormat {
    id: 105_854_261,
    puid: "wikidata/105854261",
    name: "The Witcher 2 game data archive",
    extensions: &["dzip"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x5A, 0x49, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
