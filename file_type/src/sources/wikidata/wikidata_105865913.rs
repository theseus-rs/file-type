use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865913: FileFormat = FileFormat {
    id: 105_865_913,
    source_type: SourceType::Wikidata,
    name: "The Print Shop Deluxe Graphics library",
    extensions: &["psg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x53, 0x44, 0x65, 0x6C, 0x75, 0x78, 0x65, 0x2E, 0x50, 0x53, 0x47,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
