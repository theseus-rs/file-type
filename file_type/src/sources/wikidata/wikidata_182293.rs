use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_182293: FileFormat = FileFormat {
    id: 182_293,
    source_type: SourceType::Wikidata,
    name: "LIT",
    extensions: &["lit"],
    media_types: &["application/x-ms-reader"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x54, 0x4F, 0x4C, 0x49, 0x54, 0x4C, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
