use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858254: FileFormat = FileFormat {
    id: 105_858_254,
    puid: "wikidata/105858254",
    name: "FrontPage Theme-Pack",
    extensions: &["elm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x2E, 0x30, 0x2E, 0x32, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
