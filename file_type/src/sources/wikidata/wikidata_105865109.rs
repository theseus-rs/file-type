use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865109: FileFormat = FileFormat {
    id: 105_865_109,
    source_type: SourceType::Wikidata,
    name: "DeLorme map data",
    extensions: &["pm0"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x4D, 0x61, 0x70, 0x58, 0x4D, 0x61, 0x70,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
