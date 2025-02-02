use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866672: FileFormat = FileFormat {
    id: 105_866_672,
    source_type: SourceType::Wikidata,
    name: "Pebble Draw Command sequence",
    extensions: &["pdc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x44, 0x43, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
