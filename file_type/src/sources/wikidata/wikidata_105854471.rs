use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854471: FileFormat = FileFormat {
    id: 105_854_471,
    puid: "wikidata/105854471",
    name: "AMX Mod X plugin",
    extensions: &["amxx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x58, 0x4D, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
