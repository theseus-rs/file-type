use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860082: FileFormat = FileFormat {
    id: 105_860_082,
    source_type: SourceType::Wikidata,
    name: "PV3D Value data",
    extensions: &["val"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x56, 0x33, 0x44, 0x5F, 0x56, 0x41, 0x4C, 0x55, 0x45, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
