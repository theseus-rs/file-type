use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863725: FileFormat = FileFormat {
    id: 105_863_725,
    puid: "wikidata/105863725",
    name: "LucasArts Ad Lib Audio module",
    extensions: &["laa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x44, 0x4C, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
