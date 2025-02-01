use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29465378: FileFormat = FileFormat {
    id: 29_465_378,
    puid: "wikidata/29465378",
    name: "UltraEdit project file",
    extensions: &["prj"],
    media_types: &["text/ini"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x49, 0x44, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
