use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851352: FileFormat = FileFormat {
    id: 105_851_352,
    source_type: SourceType::Wikidata,
    name: "Cura theme",
    extensions: &["json"],
    media_types: &["text/json"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x7B, 0x0A, 0x20, 0x20, 0x20, 0x20, 0x22, 0x6D, 0x65, 0x74, 0x61, 0x64, 0x61,
                    0x74, 0x61, 0x22, 0x3A, 0x20, 0x7B, 0x0A, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
                    0x20, 0x20, 0x22, 0x6E, 0x61, 0x6D, 0x65, 0x22, 0x3A, 0x20, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
