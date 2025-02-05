use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854601: FileFormat = FileFormat {
    id: 105_854_601,
    source_type: SourceType::Wikidata,
    name: "GZA compressed archive",
    extensions: &["gza"],
    media_types: &[],
    signatures: &[Signature {
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
