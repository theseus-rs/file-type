use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855683: FileFormat = FileFormat {
    id: 105_855_683,
    source_type: SourceType::Wikidata,
    name: "PSI MI format",
    extensions: &["obo"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x2D, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                    0x6E, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
