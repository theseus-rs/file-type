use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851300: FileFormat = FileFormat {
    id: 105_851_300,
    source_type: SourceType::Wikidata,
    name: "Turbo C Help",
    extensions: &["tch"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x55, 0x52, 0x42, 0x4F, 0x20, 0x43, 0x20, 0x48, 0x45, 0x4C, 0x50, 0x20,
                    0x46, 0x49, 0x4C, 0x45, 0x2E, 0x00, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
