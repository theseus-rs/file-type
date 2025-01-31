use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863576: FileFormat = FileFormat {
    id: 105_863_576,
    puid: "wikidata/105863576",
    name: "The Colony level map",
    extensions: &["1"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x41, 0x56, 0x45, 0x00, 0x01, 0x00, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
