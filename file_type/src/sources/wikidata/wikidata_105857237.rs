use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857237: FileFormat = FileFormat {
    id: 105_857_237,
    puid: "wikidata/105857237",
    name: "HTML Component (Unicode)",
    extensions: &["htc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0xFE, 0x3C, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
