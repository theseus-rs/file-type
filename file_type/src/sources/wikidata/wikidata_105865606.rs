use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865606: FileFormat = FileFormat {
    id: 105_865_606,
    source_type: SourceType::Wikidata,
    name: "Casio Prizm add-in",
    extensions: &["g3a"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xAA, 0xAC, 0xBD, 0xAF, 0x90, 0x88, 0x9A, 0x8D, 0xD3, 0xFF, 0xFE, 0xFF, 0xFE,
                    0xFF,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
