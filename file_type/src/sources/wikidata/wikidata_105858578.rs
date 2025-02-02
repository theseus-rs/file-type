use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858578: FileFormat = FileFormat {
    id: 105_858_578,
    source_type: SourceType::Wikidata,
    name: "CHDK UBASIC script (with rem)",
    extensions: &["bas"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x72, 0x65, 0x6D, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
