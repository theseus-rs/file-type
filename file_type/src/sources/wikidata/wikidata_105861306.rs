use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861306: FileFormat = FileFormat {
    id: 105_861_306,
    source_type: SourceType::Wikidata,
    name: "Linden binary Mesh",
    extensions: &["llm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x69, 0x6E, 0x64, 0x65, 0x6E, 0x20, 0x42, 0x69, 0x6E, 0x61, 0x72, 0x79,
                    0x20, 0x4D, 0x65, 0x73, 0x68, 0x20, 0x31, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
