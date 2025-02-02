use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865990: FileFormat = FileFormat {
    id: 105_865_990,
    source_type: SourceType::Wikidata,
    name: "Planner 5D Project",
    extensions: &["p5d"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x7B, 0x22, 0x68, 0x61, 0x73, 0x68, 0x22, 0x3A, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
