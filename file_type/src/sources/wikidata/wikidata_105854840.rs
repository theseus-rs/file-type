use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854840: FileFormat = FileFormat {
    id: 105_854_840,
    source_type: SourceType::Wikidata,
    name: "Binding of Isaac Rebirth packed Archive",
    extensions: &["a"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x52, 0x43, 0x48, 0x30, 0x30, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
