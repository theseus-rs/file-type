use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852094: FileFormat = FileFormat {
    id: 105_852_094,
    source_type: SourceType::Wikidata,
    name: "Complete Statistica(l) System spreadsheet (v5)",
    extensions: &["sta"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x53, 0x53, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
