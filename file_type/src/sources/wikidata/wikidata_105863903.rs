use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863903: FileFormat = FileFormat {
    id: 105_863_903,
    puid: "wikidata/105863903",
    name: "TiEmu keys Maping",
    extensions: &["map"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x6F, 0x64, 0x65, 0x6C, 0x3A, 0x20, 0x54, 0x49,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
