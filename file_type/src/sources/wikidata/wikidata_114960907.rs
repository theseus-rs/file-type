use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114960907: FileFormat = FileFormat {
    id: 114_960_907,
    source_type: SourceType::Wikidata,
    name: "Dramatica Story File",
    extensions: &["dsf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2A, 0x2A, 0x20, 0x50, 0x46, 0x4D, 0x20, 0x62, 0x69, 0x6E, 0x61, 0x72, 0x79,
                    0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x2A, 0x2A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
