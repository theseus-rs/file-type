use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863866: FileFormat = FileFormat {
    id: 105_863_866,
    puid: "wikidata/105863866",
    name: "MK-Jamz Ad Lib module",
    extensions: &["mkj"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4B, 0x4A, 0x61, 0x6D, 0x7A])],
            },
        }],
    }],
    related_formats: &[],
};
