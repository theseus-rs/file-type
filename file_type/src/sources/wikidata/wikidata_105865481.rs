use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865481: FileFormat = FileFormat {
    id: 105_865_481,
    source_type: SourceType::Wikidata,
    name: "HOT Pop-Up Program",
    extensions: &["pgm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xCD, 0x20, 0x03, 0x25, 0x03, 0x5D, 0xC3, 0x00, 0xE8,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
