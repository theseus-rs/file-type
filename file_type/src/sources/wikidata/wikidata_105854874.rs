use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854874: FileFormat = FileFormat {
    id: 105_854_874,
    source_type: SourceType::Wikidata,
    name: "Amiga Money books (v1)",
    extensions: &["amm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4D, 0x4D, 0x31, 0x42, 0x4F, 0x4B, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
