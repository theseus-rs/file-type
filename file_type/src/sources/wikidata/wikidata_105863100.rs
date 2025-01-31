use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863100: FileFormat = FileFormat {
    id: 105_863_100,
    puid: "wikidata/105863100",
    name: "AceMoney data",
    extensions: &["mmw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
