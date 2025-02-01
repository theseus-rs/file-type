use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857249: FileFormat = FileFormat {
    id: 105_857_249,
    puid: "wikidata/105857249",
    name: "Hudson Soft game data (generic)",
    extensions: &["hsf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x53, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
