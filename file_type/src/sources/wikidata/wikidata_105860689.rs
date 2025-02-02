use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860689: FileFormat = FileFormat {
    id: 105_860_689,
    source_type: SourceType::Wikidata,
    name: "RPG Maker VX Project",
    extensions: &["rvproj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x50, 0x47, 0x56, 0x58, 0x20, 0x31, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
