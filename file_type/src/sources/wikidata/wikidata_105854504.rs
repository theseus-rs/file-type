use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854504: FileFormat = FileFormat {
    id: 105_854_504,
    source_type: SourceType::Wikidata,
    name: "AUKTOOLS 2000 compressed archive",
    extensions: &["cmp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x55, 0x4B, 0x32, 0x30, 0x30, 0x30, 0x2E, 0x43, 0x4D, 0x50,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
