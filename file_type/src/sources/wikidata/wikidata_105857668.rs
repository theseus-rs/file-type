use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857668: FileFormat = FileFormat {
    id: 105_857_668,
    source_type: SourceType::Wikidata,
    name: "uBee512 DGOS tape image",
    extensions: &["tap"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x41, 0x50, 0x5F, 0x44, 0x47, 0x4F, 0x53, 0x5F, 0x4D, 0x42, 0x45, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
