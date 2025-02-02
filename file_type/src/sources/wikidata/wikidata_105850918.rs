use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850918: FileFormat = FileFormat {
    id: 105_850_918,
    source_type: SourceType::Wikidata,
    name: "Plan-80 spreadsheet (rules)",
    extensions: &["txt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3A, 0x52, 0x55, 0x4C, 0x45, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
