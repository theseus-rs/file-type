use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858606: FileFormat = FileFormat {
    id: 105_858_606,
    source_type: SourceType::Wikidata,
    name: "EVE Online data (generic)",
    extensions: &["blue"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x6C, 0x75, 0x65, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
