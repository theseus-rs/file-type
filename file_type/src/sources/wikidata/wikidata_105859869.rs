use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859869: FileFormat = FileFormat {
    id: 105_859_869,
    source_type: SourceType::Wikidata,
    name: "Knowledge Adventure MoVie video",
    extensions: &["mov"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x41, 0x4D, 0x76])],
            },
        }],
    }],
    related_formats: &[],
};
