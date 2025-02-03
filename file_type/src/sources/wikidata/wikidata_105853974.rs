use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853974: FileFormat = FileFormat {
    id: 105_853_974,
    source_type: SourceType::Wikidata,
    name: "AutoREALM Symbols",
    extensions: &["aus"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x75, 0x74, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
