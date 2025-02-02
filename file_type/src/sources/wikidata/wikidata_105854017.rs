use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854017: FileFormat = FileFormat {
    id: 105_854_017,
    source_type: SourceType::Wikidata,
    name: "AMOS source (generic)",
    extensions: &["amos"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4D, 0x4F, 0x53, 0x20, 0x42, 0x61, 0x73, 0x69, 0x63, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
