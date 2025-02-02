use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854394: FileFormat = FileFormat {
    id: 105_854_394,
    source_type: SourceType::Wikidata,
    name: "Compressed archive",
    extensions: &["cmz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x6C, 0x61, 0x79])],
            },
        }],
    }],
    related_formats: &[],
};
