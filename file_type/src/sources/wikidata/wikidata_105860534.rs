use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860534: FileFormat = FileFormat {
    id: 105_860_534,
    puid: "wikidata/105860534",
    name: "RoboForm cached data",
    extensions: &["rfo"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x72, 0x66, 0x63, 0x61, 0x63, 0x68, 0x65])],
            },
        }],
    }],
    related_formats: &[],
};
