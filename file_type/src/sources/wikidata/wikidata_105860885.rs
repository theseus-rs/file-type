use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860885: FileFormat = FileFormat {
    id: 105_860_885,
    puid: "wikidata/105860885",
    name: "Respawn Entertainment game data archive (generic)",
    extensions: &["rpak"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x50, 0x61, 0x6B])],
            },
        }],
    }],
    related_formats: &[],
};
