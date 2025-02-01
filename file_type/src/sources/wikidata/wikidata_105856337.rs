use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856337: FileFormat = FileFormat {
    id: 105_856_337,
    puid: "wikidata/105856337",
    name: "DirectWave Program",
    extensions: &["dwp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x77, 0x50, 0x72])],
            },
        }],
    }],
    related_formats: &[],
};
