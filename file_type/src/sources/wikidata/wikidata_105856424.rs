use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856424: FileFormat = FileFormat {
    id: 105_856_424,
    puid: "wikidata/105856424",
    name: "WordPerfect locked document (Amiga)",
    extensions: &["wp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFE, 0xFF, 0x61, 0x61])],
            },
        }],
    }],
    related_formats: &[],
};
