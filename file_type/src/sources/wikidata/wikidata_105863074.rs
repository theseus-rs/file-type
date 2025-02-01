use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863074: FileFormat = FileFormat {
    id: 105_863_074,
    puid: "wikidata/105863074",
    name: "VGM Music Maker module",
    extensions: &["vge"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x47, 0x45, 0x66, 0x6D, 0x74, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
