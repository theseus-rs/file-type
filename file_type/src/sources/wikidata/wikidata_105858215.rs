use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858215: FileFormat = FileFormat {
    id: 105_858_215,
    puid: "wikidata/105858215",
    name: "Everything index",
    extensions: &["db"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x5A, 0x44, 0x42, 0x06, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
