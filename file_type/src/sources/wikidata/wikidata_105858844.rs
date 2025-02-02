use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858844: FileFormat = FileFormat {
    id: 105_858_844,
    source_type: SourceType::Wikidata,
    name: "DB/TextWorks Database Term and Word Index",
    extensions: &["btx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x54, 0x58, 0x20, 0x30, 0x30, 0x35, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
