use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854581: FileFormat = FileFormat {
    id: 105_854_581,
    source_type: SourceType::Wikidata,
    name: "Microsoft(/MSN) MARC archive",
    extensions: &["mar"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x41, 0x52, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
