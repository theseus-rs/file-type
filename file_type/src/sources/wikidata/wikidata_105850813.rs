use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850813: FileFormat = FileFormat {
    id: 105_850_813,
    source_type: SourceType::Wikidata,
    name: "ED editor Keys definitions",
    extensions: &["key"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6B, 0x65, 0x79, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
