use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852737: FileFormat = FileFormat {
    id: 105_852_737,
    source_type: SourceType::Wikidata,
    name: "Superbase form",
    extensions: &["sbv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x45, 0x44, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
