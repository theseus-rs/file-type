use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860067: FileFormat = FileFormat {
    id: 105_860_067,
    source_type: SourceType::Wikidata,
    name: "Avast setup-update package",
    extensions: &["vpu"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x53, 0x57, 0x73, 0x65, 0x74, 0x75, 0x70, 0x44, 0x50, 0x6B, 0x67, 0x46,
                    0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
