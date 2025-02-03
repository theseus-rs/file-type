use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851809: FileFormat = FileFormat {
    id: 105_851_809,
    source_type: SourceType::Wikidata,
    name: "Atari ST Guide ref links",
    extensions: &["ref"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x52, 0x45, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
