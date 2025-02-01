use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854604: FileFormat = FileFormat {
    id: 105_854_604,
    puid: "wikidata/105854604",
    name: "Zzip compressed archive",
    extensions: &["zz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5A, 0x5A, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
