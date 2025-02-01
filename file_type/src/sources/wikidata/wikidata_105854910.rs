use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854910: FileFormat = FileFormat {
    id: 105_854_910,
    puid: "wikidata/105854910",
    name: "IRCAM Sound Format audio (NeXT)",
    extensions: &["sf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x64, 0xA3, 0x04, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
