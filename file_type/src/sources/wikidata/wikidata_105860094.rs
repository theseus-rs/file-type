use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860094: FileFormat = FileFormat {
    id: 105_860_094,
    source_type: SourceType::Wikidata,
    name: "Digital Video Interactive video",
    extensions: &["dvi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x44, 0x56, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
