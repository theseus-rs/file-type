use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861057: FileFormat = FileFormat {
    id: 105_861_057,
    source_type: SourceType::Wikidata,
    name: "LPMD Molecular Data",
    extensions: &["lpmd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x50, 0x4D, 0x44, 0x20, 0x32, 0x2E, 0x30, 0x20, 0x4C, 0x0A, 0x48, 0x44,
                    0x52, 0x20, 0x53, 0x59, 0x4D, 0x20, 0x58, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
