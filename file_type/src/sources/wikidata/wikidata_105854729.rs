use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854729: FileFormat = FileFormat {
    id: 105_854_729,
    puid: "wikidata/105854729",
    name: "LZIP compressed archive",
    extensions: &["lz", "lz"],
    media_types: &["application/x-lzip", "application/x-lzip"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x5A, 0x49, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
