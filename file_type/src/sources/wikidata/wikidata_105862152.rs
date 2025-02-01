use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862152: FileFormat = FileFormat {
    id: 105_862_152,
    puid: "wikidata/105862152",
    name: "MacDraft drawing",
    extensions: &["mdd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x44, 0x44, 0x43, 0x32, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
