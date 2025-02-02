use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856629: FileFormat = FileFormat {
    id: 105_856_629,
    source_type: SourceType::Wikidata,
    name: "WinWorks DB Definition",
    extensions: &["wdf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x49, 0x4E, 0x57, 0x4F, 0x52, 0x4B, 0x53, 0x20, 0x44, 0x42, 0x20, 0x44,
                    0x45, 0x46, 0x49, 0x4E, 0x49, 0x54, 0x49, 0x4F, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
