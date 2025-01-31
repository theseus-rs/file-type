use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863610: FileFormat = FileFormat {
    id: 105_863_610,
    puid: "wikidata/105863610",
    name: "Martin Fernandez Ad Lib module",
    extensions: &["adlib"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x44, 0x4C, 0x49, 0x42, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
