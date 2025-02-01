use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859284: FileFormat = FileFormat {
    id: 105_859_284,
    puid: "wikidata/105859284",
    name: "Tales Of Eternia Online game data archive (v1)",
    extensions: &["bnd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x4E, 0x4B, 0x44, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
