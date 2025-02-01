use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863395: FileFormat = FileFormat {
    id: 105_863_395,
    puid: "wikidata/105863395",
    name: "Standard 4-channel Amiga module",
    extensions: &["mod"],
    media_types: &["audio/mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x2E, 0x4B, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
