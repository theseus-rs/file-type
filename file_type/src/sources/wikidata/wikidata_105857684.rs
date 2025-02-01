use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857684: FileFormat = FileFormat {
    id: 105_857_684,
    puid: "wikidata/105857684",
    name: "KISSSlicer printer profile",
    extensions: &["ini"],
    media_types: &["text/ini"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0D, 0x0A, 0x5B])],
            },
        }],
    }],
    related_formats: &[],
};
