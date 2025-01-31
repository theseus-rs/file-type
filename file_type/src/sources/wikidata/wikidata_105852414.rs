use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852414: FileFormat = FileFormat {
    id: 105_852_414,
    puid: "wikidata/105852414",
    name: "Show.kit template",
    extensions: &["sks"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x03, 0x53, 0x4B, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
