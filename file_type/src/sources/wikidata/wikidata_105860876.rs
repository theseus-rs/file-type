use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860876: FileFormat = FileFormat {
    id: 105_860_876,
    puid: "wikidata/105860876",
    name: "RAGE Package Format (GTA IV Audio Midnight Club: LA)",
    extensions: &["rpf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x46, 0x50, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
