use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861347: FileFormat = FileFormat {
    id: 105_861_347,
    source_type: SourceType::Wikidata,
    name: "Windows logo drawing code",
    extensions: &["lgo"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x4F, 0x47, 0x4F, 0xE9])],
            },
        }],
    }],
    related_formats: &[],
};
