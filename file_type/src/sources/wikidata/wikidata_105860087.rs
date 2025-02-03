use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860087: FileFormat = FileFormat {
    id: 105_860_087,
    source_type: SourceType::Wikidata,
    name: "BCS Video",
    extensions: &["bcs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x43, 0x53, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
