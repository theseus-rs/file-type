use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854641: FileFormat = FileFormat {
    id: 105_854_641,
    puid: "wikidata/105854641",
    name: "Analysis for Windows structure",
    extensions: &["ana"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3A, 0x5C])],
            },
        }],
    }],
    related_formats: &[],
};
