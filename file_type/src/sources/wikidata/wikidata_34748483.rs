use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34748483: FileFormat = FileFormat {
    id: 34_748_483,
    puid: "wikidata/34748483",
    name: "TD0",
    extensions: &["td0"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x44, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
