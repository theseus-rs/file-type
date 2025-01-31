use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855320: FileFormat = FileFormat {
    id: 105_855_320,
    puid: "wikidata/105855320",
    name: "FlowJo Mac Workspace",
    extensions: &["jo"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x6C, 0x6F, 0x77, 0x4A, 0x6F])],
            },
        }],
    }],
    related_formats: &[],
};
