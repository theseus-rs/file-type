use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857847: FileFormat = FileFormat {
    id: 105_857_847,
    source_type: SourceType::Wikidata,
    name: "Infinity Engine exported player Character (v9.0)",
    extensions: &["chr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x48, 0x52, 0x20, 0x56, 0x39, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
