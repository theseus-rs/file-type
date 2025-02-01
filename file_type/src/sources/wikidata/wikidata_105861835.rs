use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861835: FileFormat = FileFormat {
    id: 105_861_835,
    puid: "wikidata/105861835",
    name: "Unique Development Samples",
    extensions: &["smp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x50, 0x4C, 0x53, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
