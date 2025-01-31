use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857399: FileFormat = FileFormat {
    id: 105_857_399,
    puid: "wikidata/105857399",
    name: "Lemur Module",
    extensions: &["jzlib"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x4A, 0x5A, 0x4D, 0x4C, 0x3E])],
            },
        }],
    }],
    related_formats: &[],
};
