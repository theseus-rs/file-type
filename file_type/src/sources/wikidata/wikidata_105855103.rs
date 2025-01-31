use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855103: FileFormat = FileFormat {
    id: 105_855_103,
    puid: "wikidata/105855103",
    name: "Microsoft Project 4.0 for DOS Activity",
    extensions: &["act"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x34, 0x41, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
