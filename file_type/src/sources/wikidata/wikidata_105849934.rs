use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849934: FileFormat = FileFormat {
    id: 105_849_934,
    source_type: SourceType::Wikidata,
    name: "CCS64 Cartridge",
    extensions: &["car"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x36, 0x34, 0x20, 0x43, 0x41, 0x52, 0x54, 0x52, 0x49, 0x44, 0x47, 0x45,
                    0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
