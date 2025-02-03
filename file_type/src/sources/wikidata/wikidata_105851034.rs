use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851034: FileFormat = FileFormat {
    id: 105_851_034,
    source_type: SourceType::Wikidata,
    name: "Plan-80 spreadsheet",
    extensions: &["txt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3A, 0x54, 0x49, 0x54, 0x4C, 0x45, 0x53, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
