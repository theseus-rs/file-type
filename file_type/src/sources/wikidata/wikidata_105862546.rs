use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862546: FileFormat = FileFormat {
    id: 105_862_546,
    puid: "wikidata/105862546",
    name: "MCMD module",
    extensions: &["mcmd"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x43, 0x4D, 0x44, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
