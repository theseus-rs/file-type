use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852505: FileFormat = FileFormat {
    id: 105_852_505,
    puid: "wikidata/105852505",
    name: "Superbase Program (var 1)",
    extensions: &["sbp"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x42, 0x50, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
