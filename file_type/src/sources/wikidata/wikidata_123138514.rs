use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123138514: FileFormat = FileFormat {
    id: 123_138_514,
    source_type: SourceType::Wikidata,
    name: "Disktracker Document",
    extensions: &["dtc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x64, 0x69, 0x73, 0x6B, 0x74, 0x72, 0x61, 0x6B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
