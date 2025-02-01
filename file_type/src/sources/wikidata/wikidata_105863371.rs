use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863371: FileFormat = FileFormat {
    id: 105_863_371,
    puid: "wikidata/105863371",
    name: "magic compiled data (BE)",
    extensions: &["mgc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xF1, 0x1E, 0x04, 0x1C])],
            },
        }],
    }],
    related_formats: &[],
};
