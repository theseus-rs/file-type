use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860698: FileFormat = FileFormat {
    id: 105_860_698,
    source_type: SourceType::Wikidata,
    name: "RamSoft's ZX Spectrum Replay",
    extensions: &["rzx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x5A, 0x58, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
