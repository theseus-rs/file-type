use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864172: FileFormat = FileFormat {
    id: 105_864_172,
    source_type: SourceType::Wikidata,
    name: "MPU-401 trakker module",
    extensions: &["mtk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6D, 0x70, 0x75, 0x34, 0x30, 0x31, 0x74, 0x72, 0x92, 0x6B, 0x6B, 0xEE, 0x72,
                    0x40, 0x64, 0x61, 0x74, 0x61,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
