use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851265: FileFormat = FileFormat {
    id: 105_851_265,
    source_type: SourceType::Wikidata,
    name: "SMS Triangulated Irregular Networks",
    extensions: &["tin"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x49, 0x4E])],
            },
        }],
    }],
    related_formats: &[],
};
