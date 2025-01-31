use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863210: FileFormat = FileFormat {
    id: 105_863_210,
    puid: "wikidata/105863210",
    name: "The Player 4.1a module",
    extensions: &["p41"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x34, 0x31, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
