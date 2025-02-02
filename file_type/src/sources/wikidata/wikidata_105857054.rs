use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857054: FileFormat = FileFormat {
    id: 105_857_054,
    source_type: SourceType::Wikidata,
    name: "Grid Exchange Format - ASCII",
    extensions: &["gxf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x54, 0x49, 0x54, 0x4C, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
