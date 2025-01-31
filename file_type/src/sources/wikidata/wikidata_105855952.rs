use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855952: FileFormat = FileFormat {
    id: 105_855_952,
    puid: "wikidata/105855952",
    name: "Palm Zire Photo database",
    extensions: &["db"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x42, 0x46, 0x48])],
            },
        }],
    }],
    related_formats: &[],
};
