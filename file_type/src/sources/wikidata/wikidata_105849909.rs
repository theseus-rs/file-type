use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849909: FileFormat = FileFormat {
    id: 105_849_909,
    source_type: SourceType::Wikidata,
    name: "Corel Color Palette",
    extensions: &["cpl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xDC, 0xDC])],
            },
        }],
    }],
    related_formats: &[],
};
