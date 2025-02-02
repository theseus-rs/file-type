use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205844: FileFormat = FileFormat {
    id: 28_205_844,
    source_type: SourceType::Wikidata,
    name: "COKE",
    extensions: &["tg1"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x4F, 0x4B, 0x45, 0x20, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
