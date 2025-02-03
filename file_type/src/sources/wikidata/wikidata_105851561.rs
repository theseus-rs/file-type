use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851561: FileFormat = FileFormat {
    id: 105_851_561,
    source_type: SourceType::Wikidata,
    name: "SACD multi-channel TOC",
    extensions: &["toc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x55, 0x4C, 0x43, 0x48, 0x54, 0x4F, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
