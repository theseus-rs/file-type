use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854262: FileFormat = FileFormat {
    id: 105_854_262,
    source_type: SourceType::Wikidata,
    name: "MightyFax",
    extensions: &["apf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x50, 0x46, 0x31, 0x30, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
