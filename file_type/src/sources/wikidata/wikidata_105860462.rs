use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860462: FileFormat = FileFormat {
    id: 105_860_462,
    puid: "wikidata/105860462",
    name: "R saved work space",
    extensions: &["rdata"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x44, 0x58, 0x32, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
