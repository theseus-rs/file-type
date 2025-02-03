use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861970: FileFormat = FileFormat {
    id: 105_861_970,
    source_type: SourceType::Wikidata,
    name: "PlantWALK model",
    extensions: &["model"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x6C, 0x61, 0x6E, 0x74, 0x57, 0x41, 0x4C, 0x4B, 0x20, 0x56, 0x65, 0x72,
                    0x73, 0x69, 0x6F, 0x6E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
