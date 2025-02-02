use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860860: FileFormat = FileFormat {
    id: 105_860_860,
    source_type: SourceType::Wikidata,
    name: "Taxman's Retro Engine SDK data",
    extensions: &["rsdk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x53, 0x44, 0x4B, 0x76])],
            },
        }],
    }],
    related_formats: &[],
};
