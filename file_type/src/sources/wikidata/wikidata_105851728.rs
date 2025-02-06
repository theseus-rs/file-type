use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851728: FileFormat = FileFormat {
    id: 105_851_728,
    source_type: SourceType::Wikidata,
    name: "Superbase form definition",
    extensions: &["sbv"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x44, 0x46, 0x4F, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
