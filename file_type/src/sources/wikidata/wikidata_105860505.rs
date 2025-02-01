use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860505: FileFormat = FileFormat {
    id: 105_860_505,
    puid: "wikidata/105860505",
    name: "ReadWriteThink data",
    extensions: &["rwt"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x58, 0x4D, 0x4C, 0x3E, 0x0A, 0x20, 0x20, 0x3C, 0x6F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
