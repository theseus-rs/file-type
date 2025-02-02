use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856255: FileFormat = FileFormat {
    id: 105_856_255,
    source_type: SourceType::Wikidata,
    name: "Vallen JPegger index",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x61, 0x6C, 0x6C, 0x65, 0x6E, 0x20, 0x4A, 0x50, 0x65, 0x67, 0x67, 0x65,
                    0x72, 0x20, 0x49, 0x6E, 0x64, 0x65, 0x78, 0x20, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
