use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857640: FileFormat = FileFormat {
    id: 105_857_640,
    source_type: SourceType::Wikidata,
    name: "GNU Info document",
    extensions: &["info"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
