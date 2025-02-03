use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851751: FileFormat = FileFormat {
    id: 105_851_751,
    source_type: SourceType::Wikidata,
    name: "Project Master Statistic data",
    extensions: &["sta"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x52, 0x4F, 0x4A, 0x45, 0x43, 0x54, 0x20, 0x44, 0x41, 0x54, 0x41, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
