use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854601: FileFormat = FileFormat {
    id: 105_854_601,
    puid: "wikidata/105854601",
    name: "GZA compressed archive",
    extensions: &["gza"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x47, 0x5A, 0x49, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
