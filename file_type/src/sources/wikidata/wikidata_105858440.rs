use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858440: FileFormat = FileFormat {
    id: 105_858_440,
    puid: "wikidata/105858440",
    name: "Electronic Arts LIB container",
    extensions: &["lib"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x41, 0x4C, 0x49, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
