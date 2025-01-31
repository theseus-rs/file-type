use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857243: FileFormat = FileFormat {
    id: 105_857_243,
    puid: "wikidata/105857243",
    name: "OS/2 Help (alternate)",
    extensions: &["hlp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x53, 0x50, 0x10, 0x9B, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
