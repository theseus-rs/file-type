use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866509: FileFormat = FileFormat {
    id: 105_866_509,
    puid: "wikidata/105866509",
    name: "Packed Bohemia Object game data archive",
    extensions: &["pbo"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x73, 0x72, 0x65, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
