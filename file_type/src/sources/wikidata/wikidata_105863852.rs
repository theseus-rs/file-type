use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863852: FileFormat = FileFormat {
    id: 105_863_852,
    puid: "wikidata/105863852",
    name: "Scorched Earth Mountain range data",
    extensions: &["mtn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x54, 0xBE, 0xEF, 0x00, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
