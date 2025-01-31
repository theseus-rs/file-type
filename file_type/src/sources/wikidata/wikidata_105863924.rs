use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863924: FileFormat = FileFormat {
    id: 105_863_924,
    puid: "wikidata/105863924",
    name: "MPAK game data archive",
    extensions: &["mpak"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x50, 0x41, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
