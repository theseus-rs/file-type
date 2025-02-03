use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855272: FileFormat = FileFormat {
    id: 105_855_272,
    source_type: SourceType::Wikidata,
    name: "FIGfont control file",
    extensions: &["flc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x66, 0x6C, 0x63, 0x32, 0x61])],
            },
        }],
    }],
    related_formats: &[],
};
