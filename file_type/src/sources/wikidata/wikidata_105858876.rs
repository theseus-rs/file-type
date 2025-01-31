use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858876: FileFormat = FileFormat {
    id: 105_858_876,
    puid: "wikidata/105858876",
    name: "Apple Binary 2 Library Utility archive",
    extensions: &["blu"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0A, 0x47, 0x4C, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
