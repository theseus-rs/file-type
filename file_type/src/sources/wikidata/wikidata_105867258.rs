use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867258: FileFormat = FileFormat {
    id: 105_867_258,
    source_type: SourceType::Wikidata,
    name: "Lode Data Network",
    extensions: &["ntw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x6F, 0x64, 0x65, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20, 0x4E, 0x65, 0x74,
                    0x77, 0x6F, 0x72, 0x6B, 0x20, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
