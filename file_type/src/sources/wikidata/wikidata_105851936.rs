use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851936: FileFormat = FileFormat {
    id: 105_851_936,
    puid: "wikidata/105851936",
    name: "SQLite Zipvfs compressed database",
    extensions: &["sqlite"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5A, 0x56, 0x2D])],
            },
        }],
    }],
    related_formats: &[],
};
