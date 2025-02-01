use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859588: FileFormat = FileFormat {
    id: 105_859_588,
    puid: "wikidata/105859588",
    name: "Sierra Video and Music Data video",
    extensions: &["vmd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2E, 0x03, 0x00, 0x00, 0x01, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
