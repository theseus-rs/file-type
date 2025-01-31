use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851056: FileFormat = FileFormat {
    id: 105_851_056,
    puid: "wikidata/105851056",
    name: "DemoManiac Text",
    extensions: &["txt"],
    media_types: &["txt/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x45, 0x58, 0x54, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
