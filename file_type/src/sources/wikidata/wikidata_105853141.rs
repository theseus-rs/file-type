use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853141: FileFormat = FileFormat {
    id: 105_853_141,
    source_type: SourceType::Wikidata,
    name: "Sequence Alignment/Map format (with header)",
    extensions: &["sam"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40, 0x48, 0x44, 0x09, 0x56, 0x4E, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
