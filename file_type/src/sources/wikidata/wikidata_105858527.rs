use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858527: FileFormat = FileFormat {
    id: 105_858_527,
    source_type: SourceType::Wikidata,
    name: "MIUI Backup data",
    extensions: &["bak"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x49, 0x55, 0x49, 0x20, 0x42, 0x41, 0x43, 0x4B, 0x55, 0x50, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
