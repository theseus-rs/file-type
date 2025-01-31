use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863150: FileFormat = FileFormat {
    id: 105_863_150,
    puid: "wikidata/105863150",
    name: "Mark II Soundsystem song",
    extensions: &["mii"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5A, 0x41, 0x44, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
