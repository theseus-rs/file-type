use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863894: FileFormat = FileFormat {
    id: 105_863_894,
    puid: "wikidata/105863894",
    name: "MSX MoonDriver song",
    extensions: &["mdr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x44, 0x52, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
