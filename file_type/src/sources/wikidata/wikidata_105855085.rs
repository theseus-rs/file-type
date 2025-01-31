use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855085: FileFormat = FileFormat {
    id: 105_855_085,
    puid: "wikidata/105855085",
    name: "srank compressed data",
    extensions: &["sr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x72, 0x61, 0x6E, 0x6B, 0x23])],
            },
        }],
    }],
    related_formats: &[],
};
