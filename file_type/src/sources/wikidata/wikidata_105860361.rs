use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860361: FileFormat = FileFormat {
    id: 105_860_361,
    puid: "wikidata/105860361",
    name: "Apex Legends game data archive",
    extensions: &["rpak"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x50, 0x61, 0x6B, 0x08, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
