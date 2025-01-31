use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863563: FileFormat = FileFormat {
    id: 105_863_563,
    puid: "wikidata/105863563",
    name: "Magnetic Graphics",
    extensions: &["gfx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x61, 0x50, 0x69])],
            },
        }],
    }],
    related_formats: &[],
};
