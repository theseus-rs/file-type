use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863633: FileFormat = FileFormat {
    id: 105_863_633,
    puid: "wikidata/105863633",
    name: "Rock Band multi track music",
    extensions: &["mogg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0A, 0x00, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
