use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852328: FileFormat = FileFormat {
    id: 105_852_328,
    source_type: SourceType::Wikidata,
    name: "PCLO CAD Silkscreen",
    extensions: &["silk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x49, 0x4C, 0x4B, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
