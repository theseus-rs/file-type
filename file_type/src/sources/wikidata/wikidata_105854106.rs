use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854106: FileFormat = FileFormat {
    id: 105_854_106,
    source_type: SourceType::Wikidata,
    name: "abs spreadsheet",
    extensions: &["abs"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x75, 0x62, 0x20, 0x6D, 0x61, 0x69, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
